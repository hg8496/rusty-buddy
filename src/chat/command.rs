//! This module defines traits for chat commands in a chat application.
//! The `ChatCommand` trait allows for the execution of commands,
//! taking arguments and modifying the provided `ChatService` instance.
//! The `RegisterableCommand` trait defines a method for registering
//! commands in a `CommandRegistry`, which manages available commands.
//!
//! # Traits
//!
//! ## ChatCommand
//!
//! This trait is implemented by any command that can be executed within the chat application.
//!
//! ### Usage
//!
//! To create a new chat command, implement the `ChatCommand` trait and define the `execute` method.
//!
//! ```rust
//! use crate::chat::service::ChatService;
//! use crate::chat::command::{ChatCommand, RegisterableCommand};
//! use std::error::Error;
//!
//! pub struct MyCommand;
//!
//! impl ChatCommand for MyCommand {
//!     fn execute(&self, args: &[&str], chat_service: &mut ChatService) -> Result<(), Box<dyn Error>> {
//!         // command implementation
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ## RegisterableCommand
//!
//! This trait should be implemented by commands that need to be registered in the command registry.
//!
//! ### Usage
//!
//! To register a command, implement the `RegisterableCommand` trait and use the `register_with_registry` function to associate the command with its name.
//!
//! ```rust
//! use crate::chat::command_registry::CommandRegistry;
//!
//! pub struct MyCommand;
//!
//! impl RegisterableCommand for MyCommand {
//!     fn register_with_registry(registry: &mut CommandRegistry) {
//!         registry.register_command("/mycommand", Box::new(MyCommand), vec!["my"].to_vec());
//!     }
//! }
//! ```
//!
//! ## Conclusion
//!
//! Comprehensive and clear documentation of commands helps users leverage the full capabilities of the chat application effectively.

use crate::chat::command_registry::CommandRegistry;
use crate::chat::service::ChatService;
use std::error::Error;

pub trait ChatCommand {
    fn execute(&self, args: &[&str], chat_service: &mut ChatService) -> Result<(), Box<dyn Error>>;
}

pub trait RegisterableCommand {
    fn register_with_registry(registry: &mut CommandRegistry);
}
