use std::error::Error;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use rustyline::error::ReadlineError;
use rustyline::EditMode::Emacs;
use rustyline::{Config, DefaultEditor};

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

pub fn load_files_into_context(
    directory: &Path,
    file_types: &[String], // Use a slice of strings
    context: &mut String,
) -> Result<(), Box<dyn Error>> {
    for entry in WalkDir::new(directory) {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() {
            if let Some(extension) = file_path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if file_types.contains(&ext_str.to_string()) {
                        add_to_context(context, &file_path)?;
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn add_to_context(context: &mut String, file_path: &Path) -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory for context: {}", e))?;

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path.display(), e))?;

    let relative_path = file_path
        .strip_prefix(&current_dir)
        .unwrap_or(file_path)
        .to_string_lossy();

    context.push_str(&format!(
        "Filename: {}\nContent:\n{}\n",
        relative_path, content
    ));
    Ok(())
}
