//! This module serves as an entry point for the various components of the Rusty Buddy application.
//! It organizes different modules related to commands, storage, interfaces, message handling,
//! and service management. Each submodule is responsible for specific functionality within the
//! application, promoting modular design and separation of concerns.
//!
//! ## Structure
//! - **Command Handling:** Manages user commands and interactions with the AI.
//! - **Command Registry:** Allows for dynamic registration and execution of commands.
//! - **Commands Module:** Contains various commands that Rusty Buddy can execute, such as
//!   `chat`, `commit-message`, `create-icon`, etc.
//! - **File Storage:** Manages sessions and persistent storage of chat logs and user inputs.
//! - **Interface Layer:** Facilitates communication between the chat service and backend AI models.
//! - **Message Helpers:** Provides utilities for handling messages exchanged during chats.
//! - **Service Management:** Core functionalities that support chat interactions, context management,
//!   and message processing.
mod command;
pub mod command_registry;
pub mod commands;
pub mod file_storage;
pub mod interface;
pub mod message_helpers;
pub mod service;
mod service_builder;
