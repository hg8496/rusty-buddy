//! This module provides functionality for generating commit messages
//! using the specified arguments.
//!
//! It includes the `run` function, which serves as the primary entry point
//! for executing the commit message command. The command utilizes the
//! `CommitMessageArgs` structure to define and parse the relevant command-line
//! arguments. The execution of the command is handled by the `run_commitmessage`
//! function, which performs the necessary operations to generate the
//! commit message based on the current state of the git repository.
//!
//! ## Usage
//!
//! Here's how you can execute the commit message command:
//!
//! ```rust
//! use crate::cli::commitmessage::{CommitMessageArgs, run_commitmessage};
//!
//! #[tokio::main]
//! async fn main() {
//!     let args = CommitMessageArgs { /* populate with command-line arguments */ };
//!     
//!     if let Err(e) = run_commitmessage().await {
//!         eprintln!("Error generating commit message: {}", e);
//!     }
//! }
//! ```
//!
//! In this example, we initialize `CommitMessageArgs` and run the command
//! asynchronously. It captures any errors that may occur during the
//! process, providing a clear message if something goes wrong.
//!
//! ## Important Notes
//!
//! - The command retrieves information from the git staging area to generate
//!   contextually appropriate commit messages.
//! - It is essential to ensure that changes are staged using `git add`
//!   before executing this command.
//! - The commit message follows best practices and is designed to enhance
//!   the clarity and quality of commit history in a version control system.

use crate::chat::file_storage::NilChatStorage;
use crate::chat::service::ChatService;
use crate::config;
use crate::persona::Persona;
use std::process::Command;

/// This function runs the commit message generation process.
/// It utilizes AI to create a concise summary based on the changes staged in Git.
/// The function retrieves the AI model configuration, sets up a chat service with a predefined persona,
/// and sends a message that contains a summary of changes derived from the output of `git diff`.
/// This is intended to help users create meaningful commit messages in line with best practices.
///
/// The function returns a Result indicating success or an error if the process fails.
pub async fn run_commitmessage() -> Result<(), Box<dyn std::error::Error>> {
    let model = {
        let config = config::CONFIG.lock().unwrap();
        config.ai.commit_model.clone()
    };
    let storage = NilChatStorage {};

    let mut chat_service = ChatService::builder()
        .model_name(&model)
        .storage(Box::new(storage))
        .persona(Persona {
            name: "git_expert".to_string(),
            chat_prompt: "Begin your message with a short summary of your changes (up to 50 characters as a guideline).
        Separate it from the following body by including a blank line.
        The body of your message should provide detailed answers to the following questions:
        – What was the motivation for the change?
        – How does it differ from the previous implementation?
        Use the imperative, present tense («change», not «changed» or «changes»)
        to be consistent with generated messages from commands like git merge.
    Following these rules write a concise but short commit message reflecting the following changes:
    ".to_string(),
            file_types: vec![],
        })
        .build()?;

    let diff = generate_git_diff_summary().await?;
    let summary = chat_service.send_message(&diff, false).await?;

    println!("Summary of git diff (model: {}):\n{}", &model, summary);

    Ok(())
}

pub async fn generate_git_diff_summary() -> Result<String, Box<dyn std::error::Error>> {
    // Capture the output of `git diff`
    let output = Command::new("git").arg("diff").arg("--cached").output()?;

    if !output.status.success() {
        return Err("Failed to execute git diff".into());
    }

    let diff_output = String::from_utf8_lossy(&output.stdout);

    Ok(diff_output.to_string())
}
