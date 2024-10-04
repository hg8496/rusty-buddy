use crate::chat::command::{ChatCommand, RegisterableCommand};
use crate::chat::command_registry::CommandRegistry;
use crate::chat::file_storage::DirectoryChatStorage;
use crate::chat::message_helpers::find_last_assistant_message;
use crate::chat::service::ChatService;
use crate::cli::editor::get_filename_input;
use crate::openai_api::openai_interface::OpenAIInterface;
use std::error::Error;
use std::fs;

pub struct SaveLastAnswerCommand;

impl SaveLastAnswerCommand {
    pub fn new() -> Self {
        SaveLastAnswerCommand {}
    }
}

impl ChatCommand for SaveLastAnswerCommand {
    fn execute(
        &self,
        _args: &[&str],
        chat_service: &mut ChatService<OpenAIInterface, DirectoryChatStorage>,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(last_message) = find_last_assistant_message(chat_service) {
            let default_file_name = "last_answer.txt";
            let user_file_path = get_filename_input(&format!(
                "Enter file path to save the last answer (default: {}). Use <Tab> for filename autocompletion: ",
                default_file_name
            ))?;

            let file_path = if user_file_path.trim().is_empty() {
                default_file_name.to_string()
            } else {
                user_file_path
            };

            fs::write(&file_path, last_message)?;
            println!("Last assistant answer saved to '{}'.", file_path);
        } else {
            println!("No assistant answer to save.");
        }

        Ok(())
    }
}

impl RegisterableCommand for SaveLastAnswerCommand {
    fn register_with_registry(registry: &mut CommandRegistry) {
        let command = SaveLastAnswerCommand::new();
        registry.register_command(
            "/save-last-answer",
            Box::new(command),
            vec!["save-last-answer".to_string()],
        );
    }
}
