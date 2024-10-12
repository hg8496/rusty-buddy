use crate::config::{get_knowledge_dir, CONFIG};
use crate::knowledge::{
    EmbeddingData, EmbeddingServiceBuilder, EmbeddingServiceHandle, KnowledgeResult,
    KnowledgeStore, Record,
};
use async_trait::async_trait;
use std::error::Error;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

/// Concrete implementation of the `KnowledgeStore` trait.
/// This implementation is backed by SurrealDB and generates embeddings
/// using the EmbeddingServiceHandle.
pub struct KnowledgeStoreImpl {
    embedding_service: EmbeddingServiceHandle,
    db: Surreal<Db>,
}

impl KnowledgeStoreImpl {
    /// Creates a new instance of `KnowledgeStoreImpl` by connecting to the knowledge database
    /// and initializing an `EmbeddingServiceHandle` based on the current configuration.
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        // Get the embedding model from the configuration
        let embedding_model = {
            let config = CONFIG.lock().unwrap();
            config.ai.embedding_model.clone()
        };
        // Create an embedding service based on the selected model
        let embedding_service = EmbeddingServiceBuilder::new()
            .model_name(embedding_model.into())
            .build()?;

        // Connect to the SurrealDB local database
        let db = Surreal::new::<RocksDb>(get_knowledge_dir()?.to_str().unwrap()).await?;
        db.use_ns("knowledge").use_db("knowledge_db").await?;

        // Ensure the knowledge database has an index for HNSW-based embeddings similarity search
        db.query(format!(
            "DEFINE INDEX hnsw_pts ON context_embeddings FIELDS embedding HNSW DIMENSION {};",
            embedding_service.inner.embedding_len()
        ))
        .await?;

        Ok(KnowledgeStoreImpl {
            embedding_service,
            db,
        })
    }
}

#[async_trait]
impl KnowledgeStore for KnowledgeStoreImpl {
    /// Implements the core logic for querying the knowledge database. It takes the user input, generates
    /// an embedding from it, and queries the database for relevant documents based on similarity.
    async fn query_knowledge(
        &self,
        user_input: String,
    ) -> Result<Vec<KnowledgeResult>, Box<dyn Error>> {
        // Generate the embedding for the user input
        let embedding = self
            .embedding_service
            .inner
            .get_embedding(user_input)
            .await?;

        // Query the knowledge base for the closest embeddings (most relevant documents)
        let mut results = self.db
            .query("SELECT data_source, content, metadata, vector::similarity::cosine(embedding, $embedding) AS distance FROM context_embeddings WHERE embedding <|10,40|> $embedding ORDER BY distance;")
            .bind(("embedding", embedding))
            .await?;

        // Collect file names and create relevant knowledge messages for each document found
        let files: Vec<KnowledgeResult> = results.take(0)?; // Assuming each result is a document file_name

        Ok(files)
    }

    async fn store_knowledge(&self, knowledge: EmbeddingData) -> Result<(), Box<dyn Error>> {
        self.db
            .upsert::<Option<Record>>(("context_embeddings", knowledge.data_source.to_string()))
            .content(knowledge)
            .await?;
        Ok(())
    }

    async fn get_embedding(&self, content: String) -> Result<Box<Vec<f32>>, Box<dyn Error>> {
        Ok(self.embedding_service.inner.get_embedding(content).await?)
    }
}
