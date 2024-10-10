use crate::config::{AIBackend, CONFIG};
use crate::knowledge::{EmbeddingService, EmbeddingServiceHandle};
use crate::provider::ollama::ollama_interface::OllamaInterface;
use crate::provider::openai::openai_interface::OpenAIInterface;
use log::debug;
use std::borrow::Cow;
use std::error::Error;
use std::sync::Arc;

#[derive(Default)]
pub struct EmbeddingServiceBuilder {
    model_name: Option<String>,
}

impl EmbeddingServiceBuilder {
    pub(crate) fn new() -> EmbeddingServiceBuilder {
        EmbeddingServiceBuilder::default()
    }
}

impl EmbeddingServiceBuilder {
    pub fn model_name(mut self, model_name: Cow<str>) -> Self {
        self.model_name = Some(model_name.to_string());
        self
    }

    // Build method to construct the ChatServiceFactory
    pub fn build(self) -> Result<EmbeddingServiceHandle, Box<dyn Error>> {
        // Ensure all required fields are set
        let model_name = self.model_name.ok_or("Model name must be provided.")?;
        let config = CONFIG.lock().unwrap();

        debug!("{:?}", &config);
        // Find the model details in the config
        let model = config
            .models
            .as_ref()
            .and_then(|models| models.iter().find(|m| m.name == model_name))
            .ok_or_else(|| format!("Model '{}' not found in configuration", model_name))?;
        let timeout_secs = config.ai.chat_timeout_secs;
        // Check which provider to use based on the model
        let service: Arc<dyn EmbeddingService> = match &model.backend {
            AIBackend::OpenAI => {
                Arc::new(OpenAIInterface::new(model.api_name.clone(), timeout_secs))
            } // Additional backends can be added here
            AIBackend::Ollama => Arc::new(OllamaInterface::new(
                model.api_name.clone(),
                model.url.clone(),
            )),
        };

        Ok(EmbeddingServiceHandle::new(service))
    }
}
