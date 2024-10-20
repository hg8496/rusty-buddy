//! This command renews the chat context.
//!
//! The `RenewCommand` struct implements the `ChatCommand` trait, defining how the command behaves when executed. It allows interaction with the provided `ChatService`, enabling the refreshment of the context within a chat session. This is particularly useful for reloading relevant documents or user settings.
//!
//! Additionally, the `RenewCommand` struct implements the `RegisterableCommand` trait, enabling it to register itself within a command registry. This associates the command with the provided command string `"/renew"` and its aliases, such as `"renew"`.
//!
//! In the example above, the `RenewCommand` is registered and subsequently executed to refresh the chat context.

use crate::chat::command::{ChatCommand, RegisterableCommand};
use crate::chat::command_registry::CommandRegistry;
use crate::chat::service::ChatService;
use std::error::Error;

/// A command that renews the chat context.
/// It implements `ChatCommand` to define how the
/// command behaves when executed, allowing it to
/// interact with the provided `ChatService`.
///
/// Additionally, it implements `RegisterableCommand`,
/// enabling it to register itself with a command registry,
/// associating the command with the provided command string
/// ("/renew") and its aliases (e.g., "renew").
pub struct RenewCommand {}

impl RenewCommand {
    pub fn new() -> Self {
        RenewCommand {}
    }
}

impl ChatCommand for RenewCommand {
    fn execute(&self, args: &[&str], chat_service: &mut ChatService) -> Result<(), Box<dyn Error>> {
        println!("Renewing the context...");
        if let Some(param) = args.first() {
            println!("Received parameter for renewing: {}", param);
        }

        chat_service.setup_context();
        Ok(())
    }
}

impl RegisterableCommand for RenewCommand {
    fn register_with_registry(registry: &mut CommandRegistry) {
        let command = RenewCommand::new();
        registry.register_command("/renew", Box::new(command), vec!["renew".to_string()]);
    }
}
