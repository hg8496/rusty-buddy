//! This module defines a messaging system for chat applications,
//! including message types and interfaces for sending messages and managing chat sessions.
//! It uses the `async_trait` for asynchronous operations and the `serde` library for serialization and deserialization
//! of messages. The `Message` struct encapsulates the role of the message sender and the content of the message.
//! The `ChatBackend` trait provides an interface for sending requests asynchronously and printing statistics,
//! while the `ChatStorage` trait allows loading, saving, and listing chat sessions.
//!
//! ## Key Components
//!
//! - **Message**: Represents a message exchanged between the user and the assistant,
//!   including its role (User, Assistant, etc.) and the content of the message.
//!
//! - **MessageRole**: An enum representing the different roles a message can have,
//!   allowing the user and assistant to distinguish between messages.
//!
//! - **ChatBackend**: A trait that defines methods for sending messages to the chat model,
//!   allowing different backends (e.g., OpenAI, Ollama) to implement chat functionality.
//!
//! - **ChatStorage**: A trait for managing chat sessions, allowing for session loading, saving, and listing capabilities.
//!
//! ## Examples
//!
//! Here's how you might create a new message and use the `ChatBackend` trait:
//!
//! ```rust
//! use crate::chat::interface::{Message, MessageRole, ChatBackend};
//!
//! let msg = Message {
//!     role: MessageRole::User,
//!     content: "Hello, assistant!".to_string(),
//! };
//!
//! // Assume `backend` is an instance of a type implementing ChatBackend
//! let response = backend.send_request(&[msg], false).await;
//! ```
//!
//! ## Panic Conditions
//!
//! The `send_request` method may panic if the messages passed are malformed or incorrect.
//! Ensure that messages are validated before sending them to avoid runtime errors.
//!
//! ## Conclusion
//!
//! This module provides a foundational structure for building chat applications,
//! enabling interactions with AI models in a flexible and extensible manner.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Context,
}

#[async_trait]
pub trait ChatBackend {
    async fn send_request(
        &mut self,
        messages: &[Message],
        use_tools: bool,
    ) -> Result<String, Box<dyn Error>>;
    fn print_statistics(&self);
}

pub trait ChatStorage {
    fn load_session(&mut self, session_name: &str) -> io::Result<Vec<Message>>;
    fn save_session(&self, session_name: &str, messages: &[Message]) -> io::Result<()>;
    fn list_sessions(&self) -> io::Result<Vec<String>>;
}
