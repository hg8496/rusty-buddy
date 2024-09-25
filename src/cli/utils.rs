use std::error::Error;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use rustyline::error::ReadlineError;
use rustyline::EditMode::Emacs;
use rustyline::{Config, DefaultEditor};

use crate::cli::style::configure_mad_skin;

/// Function to capture user input using rustyline with multiline support.
pub fn get_user_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    // Configure rustyline to use multiline mode
    let config = Config::builder().edit_mode(Emacs).build();
    let mut rl = DefaultEditor::with_config(config)?;
    let mut buffer = String::new();

    // Create a MadSkin for styling the prompt
    let skin = configure_mad_skin();
    // Use termimad to print a horizontal line and a colored prompt
    skin.print_text("-------------------------------");
    skin.print_text(&format!("**{}**", prompt)); // Make the prompt bold and colored

    loop {
        match rl.readline("") {
            // Pass empty string since prompt is printed separately
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

    // Add another separation after input is complete

    Ok(buffer)
}

pub fn load_files_into_context(
    directory: &Path,
    file_extension: &str,
    context: &mut String,
) -> Result<(), Box<dyn Error>> {
    for entry in WalkDir::new(directory) {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file()
            && file_path
                .extension()
                .map(|ext| ext == file_extension)
                .unwrap_or(false)
        {
            add_to_context(context, &file_path)?
        }
    }
    Ok(())
}

pub fn add_to_context(context: &mut String, file_path: &Path) -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory for context: {}", e))?;

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path.display(), e))?;

    // Create a relative path to the current directory
    let relative_path = file_path
        .strip_prefix(&current_dir)
        .unwrap_or(file_path)
        .to_string_lossy();

    // Add the filename and content to the context
    context.push_str(&format!(
        "Filename: {}
         Content:
         {}
        ",
        relative_path, content
    ));
    Ok(())
}
