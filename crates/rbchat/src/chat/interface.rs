//! This module defines a messaging system tailored for chat applications,
//! facilitating interactions between users and an AI assistant. It includes
//! message types and interfaces to send messages and manage chat sessions.
//! Utilizing the `async_trait` for asynchronous operations, alongside the
//! `serde` library for serialization and deserialization, the system supports
//! extensible message handling and session management.
//!
//! ## Key Components
//!
//! - **Message**:
//!   Encapsulates the message's role (e.g., User, Assistant) and its content.
//!   It also includes optional metadata through the `MessageInfo` struct,
//!   which provides context about the message's origin and associated details.
//!
//! - **MessageRole**:
//!   An enum that defines different roles a message may assume (e.g., System, User, Assistant),
//!   allowing for clear distinction and handling of messages based on who authored them.
//!
//! - **MessageInfo**:
//!   A structure containing additional contextual information about the message, including
//!   the timestamp of creation and various model-related metadata. This helps storing information
//!   to later identify the messages.
//!
//! - **ChatBackend**:
//!   A trait that provides an interface for sending messages to a chat model asynchronously.
//!   It facilitates the implementation of various backends, enabling flexible integrations
//!   with models (e.g., OpenAI, Ollama).
//!
//! - **ChatStorage**:
//!   A trait that facilitates management of chat sessions. It allows for loading, saving,
//!   and listing chat sessions, ensuring state persistence across application runs.
//!
//! ## Examples
//!
//! This section illustrates how to create a new message and utilize the `ChatBackend` trait:
//!
//! ## Panic Conditions
//!
//! The `send_request` method may panic under certain circumstances, such as when
//! malformed or invalid messages are passed. To mitigate this, it's essential to validate
//! each message before dispatching it, ensuring it adheres to expected structures.
//!
//! ## Conclusion
//!
//! This module provides a robust framework for building chat applications, promoting
//! seamless interactions with AI models while ensuring flexibility and extensibility in
//! message management and session handling.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageInfo {
    ContextOrigin {
        filename: String,
    },
    UserInfo {
        timestamp: DateTime<Utc>,
        image_path: Option<String>,
    },
    AssistantInfo {
        model: String,
        persona_name: String,
        prompt_token: u32,
        completion_token: u32,
        timestamp: DateTime<Utc>,
    },
    KnowledgeInfo {
        origin: String,
        distance: f32,
    }, // Add additional variants as needed
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub info: Option<MessageInfo>,
}

// Implementing the Default trait for Message
impl Default for Message {
    fn default() -> Self {
        Message {
            role: MessageRole::User, // Example default role; adjust as necessary
            content: String::new(),  // Default content is an empty string
            info: None,              // Set info to None by default
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Context,
    Knowledge,
}

#[async_trait]
pub trait ChatBackend {
    async fn send_request(
        &mut self,
        messages: &[Message],
        use_tools: bool,
    ) -> Result<Message, Box<dyn Error>>;

    fn print_statistics(&self);
}

pub trait ChatStorage {
    fn load_session(&mut self, session_name: &str) -> io::Result<Vec<Message>>;
    fn save_session(&self, session_name: &str, messages: &[Message]) -> io::Result<()>;
    fn list_sessions(&self) -> io::Result<Vec<String>>;
}
