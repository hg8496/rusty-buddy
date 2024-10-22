use crate::config::{get_knowledge_dir, CONFIG};
use crate::knowledge::{
    ConnectionMode, EmbeddingData, EmbeddingServiceBuilder, EmbeddingServiceHandle,
    KnowledgeResult, KnowledgeStore, Record,
};
use async_trait::async_trait;
use log::{info, warn};
use std::borrow::Cow;
use std::error::Error;
use std::sync::Arc;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

/// Concrete implementation of the `KnowledgeStore` trait.
/// This implementation is backed by SurrealDB and generates embeddings
/// using the EmbeddingServiceHandle.
pub struct KnowledgeStoreImpl {
    embedding_service: EmbeddingServiceHandle,
    db: Option<Arc<Surreal<Db>>>,
}

impl KnowledgeStoreImpl {
    pub async fn new(mode: ConnectionMode) -> Result<Self, Box<dyn Error>> {
        let embedding_model = {
            let config = CONFIG.lock().unwrap();
            config.ai.embedding_model.clone()
        };
        let embedding_service = EmbeddingServiceBuilder::new()
            .model_name(embedding_model.into())
            .build()?;

        let db = match mode {
            ConnectionMode::Persistent => {
                Some(connect_to_db(embedding_service.inner.embedding_len()).await?)
            }
            ConnectionMode::OnDemand => None,
        };

        Ok(KnowledgeStoreImpl {
            embedding_service,
            db,
        })
    }

    async fn connect(&self, idx_len: usize) -> Result<Arc<Surreal<Db>>, Box<dyn Error>> {
        connect_to_db(idx_len).await
    }
}

async fn connect_to_db(idx_len: usize) -> Result<Arc<Surreal<Db>>, Box<dyn Error>> {
    info!("Connecting to db");
    let db = Surreal::new::<RocksDb>(get_knowledge_dir()?.to_str().unwrap()).await?;
    db.use_ns("knowledge").use_db("knowledge_db").await?;
    db.query(format!(
        "DEFINE INDEX idx_mtree_cosine ON context_embeddings FIELDS embedding MTREE DIMENSION {} DIST COSINE TYPE F32;",
        idx_len
    ))
        .await?;
    Ok(Arc::new(db))
}

#[async_trait]
impl KnowledgeStore for KnowledgeStoreImpl {
    /// Implements the core logic for querying the knowledge database. It takes the user input, generates
    /// an embedding from it, and queries the database for relevant documents based on similarity.
    async fn query_knowledge(
        &self,
        user_input: Cow<'_, str>,
        limit: usize,
    ) -> Result<Vec<KnowledgeResult>, Box<dyn Error>> {
        // Generate the embedding for the user input
        let embedding = self
            .embedding_service
            .inner
            .get_embedding(user_input)
            .await?;
        info!("Searching for knowledge for embedding");
        let db_handle = if let Some(db) = &self.db {
            db
        } else {
            &self
                .connect(self.embedding_service.inner.embedding_len())
                .await?
        };
        let query = format!("SELECT \
        data_source, content, metadata, vector::similarity::cosine(embedding, $embedding) AS distance \
        FROM context_embeddings \
        WHERE embedding <|{}|> $embedding \
        ORDER BY distance DESC;", limit);
        // Query the knowledge base for the closest embeddings (most relevant documents)
        let mut results = match db_handle.query(query).bind(("embedding", embedding)).await {
            Ok(results) => {
                info!("Successfully searched knowledge for embedding");
                results
            }
            Err(e) => {
                warn!("Failed to search for knowledge for embedding: {}", e);
                return Err(Box::new(e));
            }
        };
        // Collect file names and create relevant knowledge messages for each document found
        let files: Vec<KnowledgeResult> = results.take(0)?;

        Ok(files)
    }

    async fn store_knowledge(&self, knowledge: EmbeddingData) -> Result<(), Box<dyn Error>> {
        let data_source = knowledge.data_source.to_string();
        info!("Storing knowledge for: {}", data_source);
        let db_handle = if let Some(db) = &self.db {
            db
        } else {
            &self
                .connect(self.embedding_service.inner.embedding_len())
                .await?
        };

        match db_handle
            .upsert::<Option<Record>>(("context_embeddings", knowledge.data_source.to_string()))
            .content(knowledge)
            .await
        {
            Ok(_) => info!("Knowledge successfully stored!"),
            Err(err) => {
                warn!("Failed to store knowledge for {}: {}", data_source, err);
                return Err(err.into());
            }
        };

        Ok(())
    }

    async fn get_embedding(&self, content: Cow<'_, str>) -> Result<Box<Vec<f32>>, Box<dyn Error>> {
        Ok(self.embedding_service.inner.get_embedding(content).await?)
    }
}
