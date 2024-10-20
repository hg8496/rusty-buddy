//! This module provides a builder pattern for constructing a `ChatService`.
//! It allows for optional configuration of the `ChatService` through a fluent API,
//! enhancing the ease of creating chat services tailored for different AI backend interactions.
//!
//! The following fields can be configured:
//! - `model_name`: The name of the AI model to be used.
//! - `storage`: A storage backend implementing the `ChatStorage` trait.
//! - `persona`: A `Persona` that defines the character of the chat interactions.
//! - `directory`: An optional directory for storing relevant data.
//!
//! The `build` method will validate that all required fields are set and create an instance of `ChatService`.
//!
//! # Errors
//!
//! The `build` method will return an error if any of the required fields are not set.
//! It will also return an error if the specified AI model cannot be found in the configuration.
use crate::chat::interface::{ChatBackend, ChatStorage};
use crate::chat::service::ChatService;
use crate::config::{AIBackend, CONFIG};
use crate::persona::Persona;
use crate::provider::ollama::ollama_interface::OllamaInterface;
use crate::provider::openai::openai_interface::OpenAIInterface;
use log::debug;
use std::error::Error;
use std::path::PathBuf;

#[derive(Default)]
pub struct ChatServiceBuilder {
    model_name: Option<String>,
    storage: Option<Box<dyn ChatStorage>>,
    persona: Option<Persona>,
    directory: Option<Vec<PathBuf>>,
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

    pub fn directory(mut self, directory: Option<Vec<PathBuf>>) -> Self {
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
        let timeout_secs = config.ai.chat_timeout_secs;
        // Check which provider to use based on the model
        let backend: Box<dyn ChatBackend> = match &model.backend {
            AIBackend::OpenAI => {
                Box::new(OpenAIInterface::new(model.api_name.clone(), timeout_secs))
            } // Additional backends can be added here
            AIBackend::Ollama => Box::new(OllamaInterface::new(
                model.api_name.clone(),
                model.url.clone(),
            )),
        };

        Ok(ChatService::new(backend, storage, persona, self.directory))
    }
}
