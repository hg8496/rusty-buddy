use crate::chat::command::{ChatCommand, RegisterableCommand};
use crate::chat::command_registry::CommandRegistry;
use crate::chat::message_helpers::find_last_assistant_message;
use crate::chat::service::ChatService;
use crate::cli::editor::get_user_input;
use arboard::Clipboard;
use regex::Regex;
use std::error::Error;

pub struct CopyFilesCommand;

impl CopyFilesCommand {
    pub fn new() -> Self {
        CopyFilesCommand {}
    }

    fn copy_to_clipboard(content: &str) -> Result<(), Box<dyn Error>> {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(content.to_string())?;
        Ok(())
    }
}

impl ChatCommand for CopyFilesCommand {
    fn execute(&self, args: &[&str], chat_service: &mut ChatService) -> Result<(), Box<dyn Error>> {
        let assistant_answer =
            find_last_assistant_message(chat_service).ok_or("No assistant message found.")?;

        let greedy_mode = args.contains(&"greedy");
        let re = Regex::new(r"```(?s)(.*?)```")?;

        if greedy_mode {
            if let Some(start) = assistant_answer.find("```") {
                if let Some(end) = assistant_answer.rfind("```") {
                    if start < end {
                        let block_content = &assistant_answer[start + 3..end].trim();
                        Self::copy_to_clipboard(block_content)?;
                    }
                }
            }
        } else {
            let mut counter = 1;
            for cap in re.captures_iter(&assistant_answer) {
                let block_content = &cap[1];
                Self::copy_to_clipboard(block_content)?;
                println!("Copied code block {}", counter);
                get_user_input("Press enter for next block: ")?;
                counter += 1;
            }
        }
        Ok(())
    }
}

impl RegisterableCommand for CopyFilesCommand {
    fn register_with_registry(registry: &mut CommandRegistry) {
        let command = CopyFilesCommand::new();
        registry.register_command(
            "/copy-files",
            Box::new(command),
            vec!["copy-files".to_string(), "copy-files greedy".to_string()],
        );
    }
}
