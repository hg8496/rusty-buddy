use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use surrealdb::sql::Thing;

/// Wrapper type for `Arc<dyn EmbeddingService>`            
pub struct EmbeddingServiceHandle {
    pub inner: Arc<dyn EmbeddingService>,
}

impl EmbeddingServiceHandle {
    pub fn new(service: Arc<dyn EmbeddingService>) -> Self {
        EmbeddingServiceHandle { inner: service }
    }
}

#[async_trait]
pub trait EmbeddingService: Send + Sync {
    async fn get_embedding(&self, content: Cow<'_, str>) -> Result<Box<Vec<f32>>, Box<dyn Error>>;
    fn embedding_len(&self) -> usize;
}

impl Clone for EmbeddingServiceHandle {
    fn clone(&self) -> Self {
        EmbeddingServiceHandle {
            inner: Arc::clone(&self.inner), // Clone the inner Arc
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ConnectionMode {
    #[default]
    OnDemand, // Open/close connection as needed
    Persistent, // Keep connection open
}

/// KnowledgeStore trait to abstract knowledge retrieval.
/// It should generate embeddings from the user input and then perform the database query
/// to retrieve relevant documents.
#[async_trait]
pub trait KnowledgeStore: Send + Sync {
    /// Queries the knowledge database and returns a vector of relevant documents
    /// based on the user's input. It generates an embedding for the input,
    /// searches the database with a similarity query, and returns the results
    /// as `Message` structs.
    ///
    /// # Arguments
    ///
    /// * `user_input` - The user input as a string, which will be used to generate the embedding.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - a vector of `Message` structs containing the relevant knowledge, or
    /// - an error if something fails.
    async fn query_knowledge(
        &self,
        user_input: Cow<'_, str>,
        limit: usize,
    ) -> Result<Vec<KnowledgeResult>, Box<dyn Error>>;
    async fn store_knowledge(&self, knowledge: EmbeddingData) -> Result<(), Box<dyn Error>>;
    async fn get_embedding(&self, content: Cow<'_, str>) -> Result<Box<Vec<f32>>, Box<dyn Error>>;
}

/// Embedding data stored in the database with file name and calculated embedding.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingData {
    pub data_source: DataSource,
    pub embedding: Vec<f32>, // Adjust size based on the model
    pub content: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataSource {
    Context(String),
    Internet(String),
    LocalFiles(String),
}

impl Display for DataSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DataSource::Context(content) => write!(f, "Context.{}", content),
            DataSource::Internet(content) => write!(f, "Internet.{}", content),
            DataSource::LocalFiles(content) => write!(f, "LocalFiles.{}", content),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KnowledgeResult {
    pub distance: f32,
    pub data_source: DataSource,
    pub content: Option<String>,
    pub metadata: Option<String>,
}
