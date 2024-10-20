//! This module provides functionality for prompting user input in a styled manner.
//! It uses the `rustyline` library to manage command line input with features such as
//! syntax highlighting, completion, and history management. The `get_user_input` function
//! allows users to enter data seamlessly while enjoying enhanced visual feedback in the terminal.
//!
//! ## Key Responsibilities
//!
//! - Prompt the user for input with a styled and colored prompt.
//! - Manage command line editing, including history and completion features.
//! - Return the user's input as a trimmed string, handling various error conditions gracefully.
//!
//! ## Example Usage
//!
//! Here's how you might prompt the user for their name:
//!
//! ```rust
//! use crate::cli::editor::get_user_input;
//!
//! let name = get_user_input("Enter your name: ")?;
//! println!("Hello, {}!", name);
//! ```
//!
//! ## Error Handling
//!
//! This module carefully manages potential errors that may arise during user input
//! reading, including interruptions or end-of-file scenarios. When errors occur,
//! the function returns an appropriate error message encapsulated in a `Box`.
use crate::cli::style::configure_mad_skin;
use rustyline::{error::ReadlineError, Config, DefaultEditor, EditMode::Emacs};

/// Prompts the user for input with a styled prompt and returns the input as a trimmed string.
///
/// # Arguments
///
/// * `prompt` - A string slice that will be displayed to the user as a prompt.
///
/// # Returns
///
/// This function returns a `Result<String, Box<dyn std::error::Error>>`, where:
///
/// - `Ok(String)` contains the user input trimmed of whitespace,
/// - `Ok(String::new())` is returned if input is interrupted or EOF is received,
/// - `Err(Box<dyn std::error::Error>)` contains an error if reading input fails.
///
/// # Example
///
/// ```
/// let input = get_user_input("Enter your name: ")?;
/// println!("Hello, {}!", input);
/// ```
pub fn get_user_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::builder().edit_mode(Emacs).build();
    let mut rl = DefaultEditor::with_config(config)?;

    // Print a styled prompt
    let skin = configure_mad_skin();
    skin.print_text("---\n");
    skin.print_text(&format!("**{}**", prompt)); // Make the prompt bold and colored

    // Read a single line of input
    match rl.readline("") {
        Ok(input) => Ok(input.trim().to_string()),
        Err(ReadlineError::Interrupted) => Ok(String::new()), // Return empty string as cancellation
        Err(ReadlineError::Eof) => Ok(String::new()),
        Err(err) => Err(Box::new(err)),
    }
}
