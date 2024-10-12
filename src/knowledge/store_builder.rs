use crate::knowledge::store_impl::KnowledgeStoreImpl;
use crate::knowledge::KnowledgeStore;
use std::error::Error;
use std::sync::Arc;

#[derive(Default)]
pub struct StoreBuilder {}

impl StoreBuilder {
    pub(crate) fn new() -> StoreBuilder {
        StoreBuilder::default()
    }
}

impl StoreBuilder {
    // Build method to construct the ChatServiceFactory
    pub async fn build(self) -> Result<Arc<dyn KnowledgeStore>, Box<dyn Error>> {
        Ok(Arc::new(KnowledgeStoreImpl::new().await?))
    }
}
