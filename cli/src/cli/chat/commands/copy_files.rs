//! This module defines the `CopyFilesCommand`, a command within a chat service that facilitates
//! copying code blocks from the last message generated by an AI assistant to the clipboard.
//! Users can choose between two operational modes for managing code block selection and copying.
//!
//! ## Modes of Operation
//!
//! 1. **Greedy Mode**: Triggered by including the "greedy" argument. In this mode, the command identifies
//!    the range from the first to the last code block in the assistant's message and automatically copies
//!    the content to the clipboard.
//!
//! 2. **Standard Mode**: The default mode that processes each code block individually. The user is prompted
//!    before moving onto the next block, allowing for controlled copying operations.
//!
//! ## Implementation Details
//!
//! The command leverages regex patterns encapsulated in the `process_code_blocks` function to identify and
//! handle code blocks within the assistant message. It utilizes the `arboard` crate for clipboard operations
//! and prompts users interactively during the standard mode to engage consent before pivoting to new blocks.
//!
//! ## Error Handling
//!
//! Errors may arise if no assistant message exists or when interacting with the system clipboard,
//! such as permissions issues. To maintain a smooth user experience, all I/O operations should
//! be managed cautiously, with clear error reporting for debugging and traceability.
//!
//! ## Example Usage
//!
//! ```rust
//! // Create an instance of CopyFilesCommand
//! let copy_files_command = CopyFilesCommand::new();
//!
//! // Execute the command in standard mode
//! copy_files_command.execute(&[], &mut chat_service).unwrap();
//! ```
//!
//! This example illustrates instantiation and execution, governed under standard mode for thoughtful interaction.

use crate::cli::chat::commands::message_files::process_code_blocks;
use rbchat::chat::command::{ChatCommand, RegisterableCommand};
use rbchat::chat::command_registry::CommandRegistry;
use rbchat::chat::message_helpers::find_last_assistant_message;
use rbchat::chat::service::ChatService;
use crate::cli::editor::get_user_input;
use arboard::Clipboard;
use std::error::Error;

/// A command to copy code blocks from the last assistant message to the clipboard.
pub struct CopyFilesCommand;

impl CopyFilesCommand {
    /// Constructs a new `CopyFilesCommand`.
    pub fn new() -> Self {
        CopyFilesCommand {}
    }

    /// Copies content to the clipboard using the `arboard` library, managing I/O operations.
    ///
    /// # Errors
    /// Returns an error if encountering issues initializing the clipboard or setting text.
    fn copy_to_clipboard(content: &str) -> Result<(), Box<dyn Error>> {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(content.to_string())?;
        Ok(())
    }
}

impl ChatCommand for CopyFilesCommand {
    /// Executes the command to copy code blocks from the last assistant message.
    ///
    /// Leverages user interaction prompts and clipboard management to facilitate the copying
    /// of identified code blocks, accommodating both the greedy and standard copying approaches.
    ///
    /// # Errors
    /// Returns an error if no assistant message is found or if clipboard operations face issues.
    fn execute(&self, args: &[&str], chat_service: &mut ChatService) -> Result<(), Box<dyn Error>> {
        // Retrieve the latest assistant response
        let assistant_answer =
            find_last_assistant_message(chat_service).ok_or("No assistant message found.")?;

        // Determine operational mode: greedy or standard
        let greedy_mode = args.contains(&"greedy");

        // Identify and process code blocks accordingly
        process_code_blocks(&assistant_answer, greedy_mode, |counter, block_content| {
            Self::copy_to_clipboard(block_content)?;

            println!("Copied code block {}", counter);
            // Await user interaction in standard mode
            get_user_input("Press <Enter> to copy the next block: ")?;

            Ok(())
        })?;

        Ok(())
    }
}

impl RegisterableCommand for CopyFilesCommand {
    /// Registers the command with the command registry under '/copy-files'.
    fn register_with_registry(registry: &mut CommandRegistry) {
        let command = CopyFilesCommand::new();
        registry.register_command(
            "/copy-files",
            Box::new(command),
            vec!["copy-files".to_string(), "copy-files greedy".to_string()],
        );
    }
}
