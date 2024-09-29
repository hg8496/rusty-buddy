use rustyline::error::ReadlineError;
use rustyline::EditMode::Emacs;
use rustyline::{Config, DefaultEditor};
use std::error::Error;

use crate::cli::style::configure_mad_skin;

// Function to capture user input using rustyline with multiline support.
pub fn get_multiline_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    let config = Config::builder().edit_mode(Emacs).build();
    let mut rl = DefaultEditor::with_config(config)?;
    let mut buffer = String::new();

    // Create a MadSkin for styling the prompt
    let skin = configure_mad_skin();
    // Use termimad to print a horizontal line and a colored prompt
    skin.print_text("---\n");
    skin.print_text(&format!("**{}**", prompt)); // Make the prompt bold and colored

    loop {
        match rl.readline("") {
            Ok(line) => {
                if line.trim_start().starts_with('/') {
                    // Exit the loop if a slash command is entered
                    buffer.push_str(&line); // Keep the command in the buffer if needed
                    break;
                }
                buffer.push_str(&line);
                buffer.push('\n');
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => return Err(Box::new(err)),
        }
    }
    Ok(buffer)
}
pub fn get_user_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::builder().edit_mode(Emacs).build();
    let mut rl = DefaultEditor::with_config(config)?;

    // Print a styled prompt
    let skin = configure_mad_skin();
    skin.print_text("---\n");
    skin.print_text(&format!("**{}**", prompt)); // Make the prompt bold and colored

    // Read a single line of input
    match rl.readline("") {
        Ok(input) => {
            // Valid input or cancel action if empty
            Ok(input.trim().to_string())
        }
        Err(ReadlineError::Interrupted) => {
            Ok(String::new()) // Return empty, indicating cancel/use default
        }
        Err(ReadlineError::Eof) => {
            Ok(String::new()) // Return empty for end-of-file signal
        }
        Err(err) => Err(Box::new(err)),
    }
}
