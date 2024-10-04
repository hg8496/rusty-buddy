use crate::chat::command::{ChatCommand, RegisterableCommand};
use crate::chat::command_registry::CommandRegistry;
use crate::chat::file_storage::DirectoryChatStorage;
use crate::chat::{
    interface::{ChatBackend, ChatStorage, MessageRole},
    service::ChatService,
};
use crate::cli::editor::{get_filename_input, get_user_input};
use crate::openai_api::openai_interface::OpenAIInterface;
use regex::Regex;
use std::error::Error;
use std::fs;

pub struct SaveFilesCommand;

impl SaveFilesCommand {
    pub fn new() -> Self {
        SaveFilesCommand {}
    }
}

impl ChatCommand for SaveFilesCommand {
    fn execute(
        &self,
        args: &[&str],
        chat_service: &mut ChatService<OpenAIInterface, DirectoryChatStorage>,
    ) -> Result<(), Box<dyn Error>> {
        let assistant_answer = match find_last_assistant_message(chat_service) {
            Some(message) => message,
            None => {
                eprintln!("No message found.");
                return Ok(());
            }
        };

        let greedy_mode = args.contains(&"greedy");

        if greedy_mode {
            // Extract everything from the first to the last triple backtick
            if let Some(start) = assistant_answer.find("```") {
                if let Some(end) = assistant_answer.rfind("```") {
                    if start < end {
                        let content = &assistant_answer[start + 3..end].trim();
                        if save_content(content).is_ok() {
                            println!("All code blocks saved in greedy mode.");
                        }
                    }
                }
            }
        } else {
            // Regex pattern to find content within triple backticks
            let re = Regex::new(r"```(?s)(.*?)```")?;
            for (index, cap) in re.captures_iter(&assistant_answer).enumerate() {
                let content = &cap[1].trim();
                println!("Found code block #{}:\n{}", index + 1, content);

                if get_user_input("Do you want to save this code block? (y/n): ")?
                    .trim()
                    .eq_ignore_ascii_case("y")
                {
                    save_content(content)?;
                } else {
                    println!("Skipped code block #{}.", index + 1);
                }
            }
        }

        Ok(())
    }
}

fn find_last_assistant_message<B: ChatBackend, S: ChatStorage>(
    chat_service: &ChatService<B, S>,
) -> Option<String> {
    let mut last_assistant_message = None;
    chat_service.process_messages(|msg| {
        if msg.role == MessageRole::Assistant {
            last_assistant_message = Some(msg.content.clone());
        }
    });
    last_assistant_message
}

fn save_content(content: &str) -> Result<(), Box<dyn Error>> {
    if content.is_empty() {
        println!("No content to save.");
        return Ok(());
    }

    let default_file_name = "extracted_content.txt";
    let user_file_path = get_filename_input(&format!(
        "Enter file path to save the content (default: {}). Use <Tab> file for autocompletion: ",
        default_file_name
    ))?;
    let file_path = if user_file_path.trim().is_empty() {
        default_file_name.to_string()
    } else {
        user_file_path
    };
    fs::write(&file_path, content)?;
    println!("Content saved as '{}'", file_path);

    Ok(())
}

impl RegisterableCommand for SaveFilesCommand {
    fn register_with_registry(registry: &mut CommandRegistry) {
        let command = SaveFilesCommand::new();
        registry.register_command(
            "/save-files",
            Box::new(command),
            vec!["save-files".to_string(), "save-files greedy".to_string()],
        );
    }
}
