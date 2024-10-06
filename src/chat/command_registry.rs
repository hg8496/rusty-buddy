use crate::chat::command::ChatCommand;
use crate::chat::service::ChatService;
use std::collections::HashMap;
use std::error::Error;

struct CommandInformation {
    command: Box<dyn ChatCommand>,
    completions: Vec<String>,
}

pub struct CommandRegistry {
    commands: HashMap<String, CommandInformation>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        CommandRegistry {
            commands: HashMap::new(),
        }
    }

    pub fn register_command(
        &mut self,
        name: &'static str,
        command: Box<dyn ChatCommand>,
        completions: Vec<String>,
    ) {
        self.commands.insert(
            name.to_string(),
            CommandInformation {
                command,
                completions,
            },
        );
    }

    pub fn get_completions(&self) -> Vec<String> {
        let mut result = Vec::new();
        for cmd in self.commands.values() {
            result.extend(cmd.completions.iter().cloned());
        }
        result
    }

    pub fn execute_command(
        &self,
        name: &str,
        args: &[&str],
        chat_service: &mut ChatService,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(command) = self.commands.get(name) {
            command.command.execute(args, chat_service)
        } else {
            Err(format!("Command '{}' not found", name).into())
        }
    }
}
