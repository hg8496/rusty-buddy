use crate::chat::command::{ChatCommand, RegisterableCommand};
use crate::chat::command_registry::CommandRegistry;
use crate::chat::message_helpers::find_last_assistant_message;
use crate::chat::service::ChatService;
use arboard::Clipboard;
use std::error::Error;

pub struct CopyLastMessageCommand;

impl CopyLastMessageCommand {
    pub fn new() -> Self {
        CopyLastMessageCommand {}
    }

    fn copy_to_clipboard(content: &str) -> Result<(), Box<dyn Error>> {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(content.to_string())?;
        Ok(())
    }
}

impl ChatCommand for CopyLastMessageCommand {
    fn execute(
        &self,
        _args: &[&str],
        chat_service: &mut ChatService,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(last_message) = find_last_assistant_message(chat_service) {
            Self::copy_to_clipboard(&last_message)?;
            println!("Last assistant message copied to clipboard.");
        } else {
            println!("No assistant message to copy.");
        }
        Ok(())
    }
}

impl RegisterableCommand for CopyLastMessageCommand {
    fn register_with_registry(registry: &mut CommandRegistry) {
        let command = CopyLastMessageCommand::new();
        registry.register_command(
            "/copy-last-message",
            Box::new(command),
            vec!["copy-last-message".to_string()],
        );
    }
}
