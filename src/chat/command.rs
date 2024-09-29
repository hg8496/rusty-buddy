use crate::chat::command_registry::CommandRegistry;
use crate::chat::file_storage::DirectoryChatStorage;
use crate::chat::service::ChatService;
use crate::openai_api::openai_interface::OpenAIInterface;
use std::error::Error;

pub trait ChatCommand {
    fn execute(
        &self,
        args: &[&str],
        chat_service: &mut ChatService<OpenAIInterface, DirectoryChatStorage>,
    ) -> Result<(), Box<dyn Error>>;
}

pub trait RegisterableCommand {
    fn register_with_registry(registry: &mut CommandRegistry);
}
