use crate::chat::command::{ChatCommand, RegisterableCommand};
use crate::chat::command_registry::CommandRegistry;
use crate::chat::message_helpers::find_last_assistant_message;
use crate::chat::service::ChatService;
use crate::cli::editor::{get_filename_input, get_user_input};
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
    fn execute(&self, args: &[&str], chat_service: &mut ChatService) -> Result<(), Box<dyn Error>> {
        let assistant_answer =
            find_last_assistant_message(chat_service).ok_or("No assistant message found.")?;

        let greedy_mode = args.contains(&"greedy");

        process_code_blocks(&assistant_answer, greedy_mode)?;

        Ok(())
    }
}

fn process_code_blocks(content: &str, greedy: bool) -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"```(?s)(.*?)```")?;

    if greedy {
        if let Some(start) = content.find("```") {
            if let Some(end) = content.rfind("```") {
                if start < end {
                    let block_content = &content[start + 3..end].trim();
                    save_block(block_content)?;
                }
            }
        }
    } else {
        for (index, cap) in re.captures_iter(content).enumerate() {
            let block_content = &cap[1];
            prompt_and_save_block(index, block_content)?;
        }
    }

    Ok(())
}

fn save_block(block_content: &str) -> Result<(), Box<dyn Error>> {
    let content_without_first_line = block_content.lines().skip(1).collect::<Vec<_>>().join("\n");
    if !content_without_first_line.is_empty() {
        save_content(&content_without_first_line)?;
    }
    Ok(())
}

fn prompt_and_save_block(index: usize, block_content: &str) -> Result<(), Box<dyn Error>> {
    println!("Found code block #{}:", index + 1);
    println!("{}", block_content);

    if get_user_input("Do you want to save this code block? (y/n): ")?
        .trim()
        .eq_ignore_ascii_case("y")
    {
        save_block(block_content)?;
    } else {
        println!("Skipped code block #{}.", index + 1);
    }
    Ok(())
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
