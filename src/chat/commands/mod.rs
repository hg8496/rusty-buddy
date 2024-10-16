//! This module is responsible for the initialization and registration of chat commands
//! in the CommandRegistry. It imports the necessary command modules and contains
//! a function `initialize_commands` that takes a mutable reference to a
//! `CommandRegistry` and registers all the chat commands by calling their
//! respective registration methods.
//!
//! The registered commands include:
//! - `RenewCommand`: Responsible for refreshing the chat state.
//! - `SaveFilesCommand`: Handles saving files related to the chat.
//! - `SaveLastAnswerCommand`: Manages saving the last response from the chat.
//!
//! ## Overview
//!
//! This module serves as the central hub for managing chat-related commands within
//! the application. It ensures that all commands are registered with the command
//! registry, which facilitates their execution based on user input during chat
//! sessions. Each command is designed to provide specific functionality, enhancing
//! user interactions and experience.
//!
//! ## Usage
//! To use the command registration functionality, invoke the `initialize_commands`
//! function with a mutable reference to a `CommandRegistry` instance.
//!
//! ## Example
//! ```rust
//! use crate::chat::command_registry::CommandRegistry;
//!
//! let mut registry = CommandRegistry::new();
//! initialize_commands(&mut registry);
//! ```

use crate::chat::command::RegisterableCommand;
use crate::chat::command_registry::CommandRegistry;
use crate::chat::commands::copy_files::CopyFilesCommand;
use crate::chat::commands::copy_last_answer::CopyLastMessageCommand;
use crate::chat::commands::refresh::RenewCommand;
use crate::chat::commands::save_files::SaveFilesCommand;
use crate::chat::commands::save_last_answer::SaveLastAnswerCommand;

mod copy_files;
mod copy_last_answer;
mod message_files;
mod refresh;
mod save_files;
mod save_last_answer;

pub fn initialize_commands(registry: &mut CommandRegistry) {
    // Each command registers itself
    RenewCommand::register_with_registry(registry);
    SaveFilesCommand::register_with_registry(registry);
    SaveLastAnswerCommand::register_with_registry(registry);
    CopyFilesCommand::register_with_registry(registry);
    CopyLastMessageCommand::register_with_registry(registry);
}
