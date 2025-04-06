//! This module provides functionality for executing commands based on user-defined wishes within
//! the Rusty Buddy application. It allows users to express their wishes as natural language commands
//! and facilitates automatic file creation and manipulation as per the user's requests.
//! Utilizing the command-line arguments parsed into a `WishArgs` structure, it streamlines the interaction
//! between the user and the AI backend, making it easier to fulfill development tasks through simple commands.
//!
//! ## Key Components
//!
//! - **run:** An asynchronous function that takes `WishArgs` as input and delegates execution to
//!   the underlying `run_wish` function located in the `run` module. This central command processes
//!   the user's requests and interacts with the chat service to carry out the specified operations.
//!
//! ## Usage Example
//!
//! Here’s how to utilize the `run` function to execute a wish command:
//!
//! ```rust
//! use crate::cli::wish::{WishArgs, run};
//!
//! #[tokio::main]
//! async fn main() {
//!     let args = WishArgs {
//!         directory: "./src".to_string(),
//!         tools: true,
//!     };
//!     
//!     if let Err(e) = run(args).await {
//!         eprintln!("Error executing wish command: {}", e);
//!     }
//! }
//! ```
//!
//! This example demonstrates how to specify the directory and tool options,
//! allowing Rusty Buddy to fulfill the user’s request automatically, enhancing productivity
//! and efficiency in the development process.
//!
//! ## Error Handling
//!
//! The `run` function returns a `Result` type, encapsulating either a successful execution
//! (`Ok(())`) or an error wrapped in a `Box` that indicates what went wrong during
//! the request processing. Users should be prepared to handle errors gracefully to ensure
//! a smooth interaction experience.
use crate::cli::editor::get_multiline_input;
use rbchat::chat::file_storage::NilChatStorage;
use rbchat::chat::service::ChatService;
use rbchat::config;
use rbchat::persona::get_persona;
use std::error::Error;
use std::path::PathBuf;

/// Runs the wish command.
///
/// This function checks if the provided directory is valid, initializes a chat service
/// with a specified AI model and persona, and prompts the user for a wish. It sends
/// the user’s wish to the chat service and prints the AI's response. If the directory
/// is invalid, it prints an error message and exits gracefully.
///
/// # Arguments
///
/// * `directory` - A string slice that holds the path to the directory where the service is to be initialized.
/// * `use_tools` - A boolean indicating whether tools should be used in the service response.
///
/// # Errors
///
/// Returns an error wrapped in a Box if any operation fails, including directory validation,
/// user input reading, or chat service operations.
pub async fn run_wish(
    directory: Option<Vec<PathBuf>>,
    use_tools: bool,
) -> Result<(), Box<dyn Error>> {
    // Initialize the chat service with an OpenAI backend and NilStorage
    let (model, default_persona) = get_config();
    let storage = NilChatStorage {};
    let persona = get_persona(default_persona.as_str()).unwrap();

    let mut chat_service = ChatService::builder()
        .model_name(model.as_str())
        .storage(Box::new(storage))
        .persona(persona.clone())
        .directory(directory)
        .build()?;
    chat_service.setup_context();
    // Get user input for their wish
    let user_input = get_multiline_input("What do you wish? ", vec![])
        .map_err(|e| format!("Failed to read user input: {}", e))?;
    let wish = format!("Users wish: {}", user_input);

    // Send the wish to the chat service and get the AI response
    let response = chat_service
        .send_message(std::borrow::Cow::Borrowed(&wish), &None, use_tools)
        .await?;

    // Print the response and statistics
    println!("AI Context Response: {}", response);
    chat_service.print_statistics();

    Ok(())
}

fn get_config() -> (String, String) {
    let config = config::CONFIG.lock().unwrap();
    let model = config.ai.wish_model.clone();
    let default_persona = config.default_persona.clone();
    (model, default_persona)
}
