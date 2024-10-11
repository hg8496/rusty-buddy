use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

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
    async fn get_embedding(&self, content: String) -> Result<Box<Vec<f32>>, Box<dyn Error>>;
    fn embedding_len(&self) -> usize;
}

impl Clone for EmbeddingServiceHandle {
    fn clone(&self) -> Self {
        EmbeddingServiceHandle {
            inner: Arc::clone(&self.inner), // Clone the inner Arc
        }
    }
}
