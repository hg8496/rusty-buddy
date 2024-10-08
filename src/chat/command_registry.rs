use crate::chat::command::ChatCommand;
use crate::chat::service::ChatService;
use std::collections::HashMap;
use std::error::Error;

/// The `CommandRegistry` struct is responsible for managing chat commands in a chat application.
/// It allows for the registration of commands with their associated names and completions,
/// as well as providing a way to retrieve completion suggestions and execute commands.
///
/// This struct utilizes a `HashMap` to store commands, where each command is represented
/// by an instance of `CommandInformation` that contains the command itself
/// (captured as a trait object implementing `ChatCommand`) and a list of potential completions.
///
/// The `CommandRegistry` offers the following functionalities:
/// - `new`: Creates a new empty `CommandRegistry`.
/// - `register_command`: Adds a new command to the registry with a specified name and its completions.
/// - `get_completions`: Returns a list of all possible completions for registered commands.
/// - `execute_command`: Executes a registered command by its name with the provided arguments, interacting with a `ChatService`.
///
/// # Usage
///
/// To create a new command registry and register commands:
///
/// ```rust
/// use crate::chat::command_registry::CommandRegistry;
/// use crate::chat::command::ChatCommand;
///
/// // Create a new command registry
/// let mut registry = CommandRegistry::new();
///
/// // Register a command (assuming MyCommand implements ChatCommand)
/// registry.register_command("/mycommand", Box::new(MyCommand), vec!["my", "cmd"]);
/// ```
///
/// # Completion
///
/// Commands can provide completion suggestions, which can be retrieved using:
///
/// ```rust
/// let completions = registry.get_completions();
/// ```
///
/// # Execution
///
/// To execute a command with arguments:
///
/// ```rust
/// registry.execute_command("/mycommand", &["arg1", "arg2"], &mut chat_service)?;
/// ```
///
/// # Error Handling
///
/// If a command is not found, `execute_command` will return an error:
///
/// ```rust
/// Err(format!("Command '{}' not found", name).into())
/// ```
///
/// This error handling allows the caller to manage unrecognized commands gracefully, enhancing user experience.

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
