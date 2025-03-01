//! This module provides an interface for communicating with the Ollama chat model.
//! It encapsulates the functionality required to send messages to the Ollama AI backend,
//! receive responses, and manage the flow of conversation within the Rusty Buddy application.
//!
//! The `OllamaInterface` struct acts as a bridge between the chat service and the Ollama API,
//! allowing for easy interaction with the model while maintaining session context and message history.
//!
//! ## Key Responsibilities
//!
//! - **Message Handling:** Converts application-specific message formats into the format required by the Ollama API.
//! - **Session Management:** Retains state and context for ongoing conversations, facilitating a natural dialog flow.
//! - **Backend Integration:** Implements the `ChatBackend` trait to integrate seamlessly with other components in the chat ecosystem.
//!
//! ## Fields
//!
//! - `ollama`: An instance of the `Ollama` struct that handles interactions with the Ollama API.
//! - `model`: A string that specifies the model to be used for generating chat messages.
//!
//! ## Methods
//!
//! - `new`: Creates a new instance of `OllamaInterface`, initializing it with the provided model and optional URL.
//! - `convert_messages`: Converts an array of `Message` objects into `ChatMessage` objects for processing by the Ollama API.
//!
//! ## Trait Implementations
//!
//! - `ChatBackend`: Implements the necessary methods to send requests to the chat model and print statistics about the model in use.

use crate::chat::interface::{ChatBackend, Message, MessageInfo, MessageRole};
use crate::knowledge::EmbeddingService;
use async_trait::async_trait;
use chrono::Utc;
use log::{debug, error, info}; // Ensure to import appropriate logging macros
use ollama_rs::generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest};
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage},
    IntoUrlSealed, Ollama,
};
use std::borrow::Cow;
use std::error::Error;

/// OllamaInterface is a struct that provides an interface to communicate with the Ollama chat model.
/// It implements the ChatBackend trait which allows sending and receiving messages from the chat model.
#[derive(Clone)]
pub struct OllamaInterface {
    ollama: Ollama,
    model: String,
}

impl OllamaInterface {
    pub fn new(model: String, ourl: Option<String>) -> Self {
        let url = ourl.unwrap_or("http://localhost:11434".into());
        let ollama = Ollama::from_url(url.clone().into_url().unwrap());
        info!(
            "Creating Ollama interface with model: {} and URL: {}",
            model, url
        );
        OllamaInterface { ollama, model }
    }

    fn convert_messages(messages: &[Message]) -> Vec<ChatMessage> {
        messages
            .iter()
            .map(|msg| match msg.role {
                MessageRole::User => ChatMessage::user(msg.content.clone()),
                MessageRole::Assistant => ChatMessage::assistant(msg.content.clone()),
                MessageRole::Context | MessageRole::System | MessageRole::Knowledge => {
                    ChatMessage::system(msg.content.clone())
                }
            })
            .collect()
    }
}

#[async_trait]
impl ChatBackend for OllamaInterface {
    async fn send_request(
        &mut self,
        messages: &[Message],
        _use_tools: bool,
    ) -> Result<Message, Box<dyn Error>> {
        info!("Sending request to Ollama with {} messages", messages.len());
        let chat_messages = Self::convert_messages(messages);
        debug!("Converted messages for Ollama: {:?}", chat_messages);

        let request = ChatMessageRequest::new(self.model.clone(), chat_messages);
        info!(
            "Sending chat message request to Ollama for model: {}",
            self.model
        );

        match self.ollama.send_chat_messages(request).await {
            Ok(response) => {
                info!("Received response from Ollama");
                let assistant_message = response.message;
                debug!("Got assistant message: {}", assistant_message.content);
                let content_len = assistant_message.content.len();
                Ok(Message {
                    role: MessageRole::Assistant,
                    content: assistant_message.content,
                    info: Some(MessageInfo::AssistantInfo {
                        model: self.model.clone(),
                        persona_name: String::new(),
                        prompt_token: 0,
                        completion_token: content_len as u32,
                        timestamp: Utc::now(),
                    }),
                })
            }
            Err(e) => {
                error!("Failed to get response from Ollama: {}", e);
                Err(e.into())
            }
        }
    }

    fn print_statistics(&self) {
        debug!("Using Ollama model: {}", self.model);
    }
}

#[async_trait]
impl EmbeddingService for OllamaInterface {
    async fn get_embedding(&self, content: Cow<'_, str>) -> Result<Box<Vec<f32>>, Box<dyn Error>> {
        info!(
            "Generating embedding for content of length {}",
            content.len()
        );
        let request = GenerateEmbeddingsRequest::new(
            self.model.clone(),
            EmbeddingsInput::from(content.into_owned()),
        );

        match self.ollama.generate_embeddings(request).await {
            Ok(result) => {
                info!("Successfully generated embeddings");
                Ok(Box::new(result.embeddings[0].clone()))
            }
            Err(e) => {
                error!("Error generating embeddings: {}", e);
                Err(e.into())
            }
        }
    }

    fn embedding_len(&self) -> usize {
        1024
    }
}
