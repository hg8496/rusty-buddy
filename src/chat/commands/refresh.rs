use crate::chat::command::{ChatCommand, RegisterableCommand};
use crate::chat::command_registry::CommandRegistry;
use crate::chat::file_storage::DirectoryChatStorage;
use crate::chat::service::ChatService;
use crate::openai_api::openai_interface::OpenAIInterface;
use std::error::Error;

pub struct RenewCommand {}

impl RenewCommand {
    pub fn new() -> Self {
        RenewCommand {}
    }
}

impl ChatCommand for RenewCommand {
    fn execute(
        &self,
        args: &[&str],
        chat_service: &mut ChatService<OpenAIInterface, DirectoryChatStorage>,
    ) -> Result<(), Box<dyn Error>> {
        println!("Renewing the context...");
        if let Some(param) = args.get(0) {
            println!("Received parameter for renewing: {}", param);
        }

        chat_service.setup_context();
        Ok(())
    }
}

impl<'a> RegisterableCommand for RenewCommand {
    fn register_with_registry(registry: &mut CommandRegistry) {
        let command = RenewCommand::new();
        registry.register_command("/renew", Box::new(command));
    }
}
