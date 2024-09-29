use crate::chat::command::ChatCommand;
use crate::chat::file_storage::DirectoryChatStorage;
use crate::chat::service::ChatService;
use crate::openai_api::openai_interface::OpenAIInterface;
use std::collections::HashMap;
use std::error::Error;

pub struct CommandRegistry<'a> {
    commands: HashMap<&'static str, Box<dyn ChatCommand + 'a>>,
}

impl<'a> CommandRegistry<'a> {
    pub fn new() -> Self {
        CommandRegistry {
            commands: HashMap::new(),
        }
    }

    pub fn register_command(&mut self, name: &'static str, command: Box<dyn ChatCommand + 'a>) {
        self.commands.insert(name, command);
    }

    pub fn execute_command(
        &self,
        name: &str,
        args: &[&str],
        chat_service: &mut ChatService<OpenAIInterface, DirectoryChatStorage>,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(command) = self.commands.get(name) {
            command.execute(args, chat_service)
        } else {
            Err(format!("Command '{}' not found", name).into())
        }
    }
}
