//! This module defines the `SaveFilesCommand`, a command within a chat service
//! that allows users to save code blocks extracted from the last assistant's message.
//! It supports two modes of operation to accommodate different user preferences.
//!
//! ## Modes of Operation
//!
//! 1. **Greedy Mode**: Activated by including the "greedy" argument. In this mode,
//!    the command finds and saves the content between the first and last code blocks
//!    identified in the assistant's message.
//!
//! 2. **Standard Mode**: The default mode when no arguments are provided. It iterates
//!    over all individual code blocks in the message. The user is prompted to confirm
//!    whether to save each block, allowing selective saving of desired content.
//!
//! ## Implementation Details
//!
//! The command employs a regular expression within the `process_code_blocks` function
//! to identify code blocks in the message. It then uses closures to handle the individual
//! processing of these blocks. User input prompts are generated to confirm whether to save
//! detected blocks, leveraging Rust's I/O functionalities for this interaction.
//!
//! ## Error Handling
//!
//! The command will generate an error if no assistant message is available or if any
//! file operations, such as writing to a file, encounter issues like invalid paths
//! or inadequate permissions. It is crucial to handle these errors in your implementation
//! to ensure a robust user experience.
//!
//! ## Example Usage
//!
//! ```rust
//! // Create an instance of SaveFilesCommand
//! let save_files_command = SaveFilesCommand::new();
//!
//! // Execute the command in greedy mode to save code blocks as specified
//! save_files_command.execute(&["greedy"], &mut chat_service).unwrap();
//! ```
//!
//! This example demonstrates instantiating and executing the command.
//! It assumes prior initialization of a chat service context.

use crate::cli::chat::commands::message_files::process_code_blocks;
use crate::cli::editor::{get_filename_input, get_user_input};
use rbchat::chat::command::{ChatCommand, RegisterableCommand};
use rbchat::chat::command_registry::CommandRegistry;
use rbchat::chat::message_helpers::find_last_assistant_message;
use rbchat::chat::service::ChatService;
use std::error::Error;
use std::fs;

/// Represents a command to save code blocks from the last assistant message in a chat service.
pub struct SaveFilesCommand;

impl SaveFilesCommand {
    /// Constructs a new `SaveFilesCommand`.
    pub fn new() -> Self {
        SaveFilesCommand {}
    }
}

impl ChatCommand for SaveFilesCommand {
    /// Executes the command to save code blocks of the last assistant message.
    ///
    /// Identifies the code blocks using regular expressions and prompts
    /// the user to save them, either by default interaction (Standard Mode)
    /// or automatically in a greedy fashion (Greedy Mode).
    ///
    /// # Errors
    /// Returns an error if no assistant message is available or if saving the file fails.
    fn execute(&self, args: &[&str], chat_service: &mut ChatService) -> Result<(), Box<dyn Error>> {
        // Retrieve the last assistant message
        let assistant_answer =
            find_last_assistant_message(chat_service).ok_or("No assistant message found.")?;

        // Determine the mode (greedy or standard)
        let greedy_mode = args.contains(&"greedy");

        // Process code blocks based on the determined mode
        process_code_blocks(&assistant_answer, greedy_mode, |pos, block_content| {
            if !block_content.is_empty() {
                prompt_and_save_block(pos, block_content)?;
            }
            Ok(())
        })?;

        Ok(())
    }
}

/// Prompts the user to save a specific code block and writes it to a file if confirmed.
fn prompt_and_save_block(index: usize, block_content: &str) -> Result<(), Box<dyn Error>> {
    println!("Found code block #{}:", index);

    // Request user confirmation to save the code block
    if get_user_input("Do you want to save this code block? (y/n): ")?
        .trim()
        .eq_ignore_ascii_case("y")
    {
        save_content(block_content)?;
    } else {
        println!("Skipped code block #{}.", index + 1);
    }
    Ok(())
}

/// Writes the provided content to a file specified by the user or defaults to a predefined filename.
fn save_content(content: &str) -> Result<(), Box<dyn Error>> {
    // Default file name for saving content if the user does not provide one
    let default_file_name = "extracted_content.txt";
    let user_file_path = get_filename_input(&format!(
        "Enter file path to save the content (default: {}). Use <Tab> for file autocompletion: ",
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
    /// Registers the command with the given command registry under '/save-files'.
    fn register_with_registry(registry: &mut CommandRegistry) {
        let command = SaveFilesCommand::new();
        registry.register_command(
            "/save-files",
            Box::new(command),
            vec!["save-files".to_string(), "save-files greedy".to_string()],
        );
    }
}
