use crate::knowledge::store_impl::KnowledgeStoreImpl;
use crate::knowledge::{ConnectionMode, KnowledgeStore};
use std::error::Error;
use std::sync::Arc;

#[derive(Default)]
pub struct StoreBuilder {
    connection_mode: ConnectionMode,
}

impl StoreBuilder {
    pub(crate) fn new() -> StoreBuilder {
        StoreBuilder::default()
    }
}

impl StoreBuilder {
    // Build method to construct the ChatServiceFactory
    pub async fn build(&self) -> Result<Arc<dyn KnowledgeStore>, Box<dyn Error>> {
        Ok(Arc::new(
            KnowledgeStoreImpl::new(self.connection_mode).await?,
        ))
    }

    pub fn connection_mode(&mut self, mode: ConnectionMode) -> &mut Self {
        self.connection_mode = mode;
        self
    }
}
