use crate::cli::style::configure_mad_skin;
use rustyline::{error::ReadlineError, Config, DefaultEditor, EditMode::Emacs};

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
        Err(ReadlineError::Interrupted) => Ok(String::new()), // Return empty string as cancelation
        Err(ReadlineError::Eof) => Ok(String::new()),
        Err(err) => Err(Box::new(err)),
    }
}
