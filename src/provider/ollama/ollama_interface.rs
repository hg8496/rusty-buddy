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
//! ## Example Usage
//!
//! ```rust
//! use crate::provider::ollama::ollama_interface::OllamaInterface;
//!
//! let ollama_backend = OllamaInterface::new("llama2".to_string(), None);
//! // Here “llama2” is an example model name
//! ```
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
use crate::chat::interface::{ChatBackend, Message, MessageRole};
use async_trait::async_trait;
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponse},
    IntoUrlSealed, Ollama,
};
use std::error::Error;

/// OllamaInterface is a struct that provides an interface to communicate with the Ollama chat model.
/// It implements the ChatBackend trait which allows sending and receiving messages from the chat model.
pub struct OllamaInterface {
    ollama: Ollama,
    model: String,
}

impl OllamaInterface {
    pub fn new(model: String, ourl: Option<String>) -> Self {
        let url = ourl.unwrap_or("http://localhost:11434".into());
        OllamaInterface {
            ollama: Ollama::from_url(url.clone().into_url().unwrap()),
            model,
        }
    }

    fn convert_messages(messages: &[Message]) -> Vec<ChatMessage> {
        let mut chat_messages: Vec<ChatMessage> = Vec::new();

        // Convert Message into ChatMessage for ollama
        for msg in messages {
            match msg.role {
                MessageRole::User => {
                    chat_messages.push(ChatMessage::user(msg.content.clone()));
                }
                MessageRole::Assistant => {
                    chat_messages.push(ChatMessage::assistant(msg.content.clone()));
                }
                MessageRole::Context => {
                    chat_messages.push(ChatMessage::system(msg.content.clone()));
                }
                MessageRole::System => {
                    chat_messages.push(ChatMessage::system(msg.content.clone()));
                }
            }
        }
        chat_messages
    }
}

#[async_trait]
impl ChatBackend for OllamaInterface {
    async fn send_request(
        &mut self,
        messages: &[Message],
        _use_tools: bool,
    ) -> Result<String, Box<dyn Error>> {
        let chat_messages = Self::convert_messages(messages);

        let request = ChatMessageRequest::new(self.model.clone(), chat_messages.clone());
        let responseo: ChatMessageResponse = self.ollama.send_chat_messages(request).await?;

        let mut content = String::new();

        if let Some(assistant_message) = responseo.message {
            content += &assistant_message.content;
        }
        Ok(content)
    }

    fn print_statistics(&self) {
        // Implement statistics if required
        println!("Using Ollama model: {}", self.model);
    }
}
