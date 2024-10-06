use crate::chat::command_registry::CommandRegistry;
use crate::chat::service::ChatService;
use std::error::Error;

pub trait ChatCommand {
    fn execute(&self, args: &[&str], chat_service: &mut ChatService) -> Result<(), Box<dyn Error>>;
}

pub trait RegisterableCommand {
    fn register_with_registry(registry: &mut CommandRegistry);
}
