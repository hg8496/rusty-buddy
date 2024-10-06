use crate::chat::interface::{ChatBackend, ChatStorage};
use crate::chat::service::ChatService;
use crate::config::{AIBackend, CONFIG};
use crate::persona::Persona;
use crate::provider::openai::openai_interface::OpenAIInterface;
use log::debug;
use std::error::Error;

// Builder struct
#[derive(Default)]
pub struct ChatServiceBuilder {
    model_name: Option<String>,
    storage: Option<Box<dyn ChatStorage>>,
    persona: Option<Persona>,
    directory: Option<String>,
}

impl ChatServiceBuilder {
    pub fn model_name(mut self, model_name: &str) -> Self {
        self.model_name = Some(model_name.to_string());
        self
    }

    pub fn storage(mut self, storage: Box<dyn ChatStorage>) -> Self {
        self.storage = Some(storage);
        self
    }

    pub fn persona(mut self, persona: Persona) -> Self {
        self.persona = Some(persona);
        self
    }

    pub fn directory(mut self, directory: Option<String>) -> Self {
        self.directory = directory;
        self
    }

    // Build method to construct the ChatServiceFactory
    pub fn build(self) -> Result<ChatService, Box<dyn Error>> {
        // Ensure all required fields are set
        let model_name = self.model_name.ok_or("Model name must be provided.")?;
        let storage = self.storage.ok_or("Storage must be provided.")?;
        let persona = self.persona.ok_or("Persona must be provided.")?;
        let config = CONFIG.lock().unwrap();

        debug!("{:?}", &config);
        // Find the model details in the config
        let model = config
            .models
            .as_ref()
            .and_then(|models| models.iter().find(|m| m.name == model_name))
            .ok_or_else(|| format!("Model '{}' not found in configuration", model_name))?;

        // Check which provider to use based on the model
        let backend: Box<dyn ChatBackend> = match &model.backend {
            AIBackend::OpenAI => Box::new(OpenAIInterface::new(model.api_name.clone())), // Additional backends can be added here
            _ => return Err(format!("Unknown backend for model: {:?}", model.backend).into()),
        };

        Ok(ChatService::new(backend, storage, persona, self.directory))
    }
}
