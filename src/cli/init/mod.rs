//! This module provides the functionality needed for the initialization
//! of the Rusty Buddy application. It primarily focuses on configuring
//! the AI backend, managing user inputs for API keys and model types,
//! and setting up environment variables and configuration files.
//!
//! The entry point is the `run_init_command` function, which guides the
//! user through the setup process and ensures that the application is
//! ready for use with the selected AI backend (OpenAI or Ollama).
//
//! ## Key Functions
//!
//! - `run_init_command`: This asynchronous function is responsible for
//!   initializing the application, managing the choice of AI backend,
//!   and creating the necessary configuration files. It prompts the user
//!   for essential details like API keys and model selections.
//! - `choose_backend_option`: Prompts the user to select their desired
//!   backend (OpenAI or Ollama) and returns the choice.
//! - `get_or_prompt_openai_key`: Retrieves the OpenAI API key from
//!   environment variables or prompts the user to enter it if not found.
//! - `write_openai_key_to_env_file`: Saves the provided OpenAI API key
//!   to the `.env` file for future use.
//! - `get_directory_listing`: Returns a list of files in the specified
//!   directory, formatted as a string.
//! - `get_internal_personas`: Retrieves the internal persona configurations,
//!   providing available options for user interactions with the AI.
//!
//! ## Example Usage
//!
//! The typical workflow when initializing the application would involve
//! running the command:
//!
//! ```
//! rusty-buddy init
//! ```
//!
//! This command initiates the setup process, allowing users to configure
//! the tool according to their specific development needs.
use crate::chat::file_storage::NilChatStorage;
use crate::chat::interface::ChatBackend;
use crate::chat::service::ChatService;
use crate::cli::editor::{get_password_input, get_user_input};
use crate::config::AIBackend;
use crate::persona::{get_internal_persona_configs, Persona};
use crate::provider::ollama::ollama_interface::OllamaInterface;
use crate::provider::openai::openai_interface::OpenAIInterface;
use dotenvy::dotenv;
use std::error::Error;
use std::io::Write;
use std::{env, fs};
use walkdir::WalkDir;

/// This function represents the entry point of the init command.
/// It initializes the configuration based on user choice of AI backend (OpenAI or Ollama),
/// prompts for necessary configuration details (like API keys and model types),
/// and sets up the environment and configuration files accordingly.
pub async fn run_init_command() -> Result<(), Box<dyn Error>> {
    // Load existing environment variables
    dotenv().ok();

    // Ask the user which backend to use
    let backend_choice = choose_backend_option()?;
    let files = get_directory_listing(".");
    let personas = get_internal_personas();

    match backend_choice {
        AIBackend::OpenAI => {
            println!("You chose OpenAI.");
            let openai_key = get_or_prompt_openai_key()?;
            write_openai_key_to_env_file(&openai_key)?;
            let backend = OpenAIInterface::new("gpt-4o-mini".to_string(), 60);
            let recommended_persona = recommend_persona(files, personas, Box::new(backend)).await?;
            write_config(
                &recommended_persona,
                "openai_complex",
                "openai_fast",
                "http://localhost:11434",
                "llama3.2",
            )?;
            println!(
                "Configuration successfully initialized with persona: {}",
                recommended_persona
            );
        }
        AIBackend::Ollama => {
            println!("You chose Ollama.");
            let mut ollama_url = get_user_input(
                "Please enter the Ollama API URL (default: http://localhost:11434): ",
            )?;
            if ollama_url.is_empty() {
                ollama_url = "http://localhost:11434".to_string();
            }
            let mut model = get_user_input("Please enter the Ollama model (default: llama3.2")?;
            if model.is_empty() {
                model = "llama3.2".to_string();
            }
            let backend = OllamaInterface::new(model.clone(), Some(ollama_url.clone()));
            let recommended_persona = recommend_persona(files, personas, Box::new(backend)).await?;
            write_config(
                &recommended_persona,
                "ollama_complex",
                "ollama_complex",
                ollama_url.as_str(),
                model.as_str(),
            )?;
            println!(
                "Configuration successfully initialized with persona: {}",
                recommended_persona
            );
        }
    }

    Ok(())
}

// Function to recommend a persona
async fn recommend_persona(
    dir_listing: String,
    personas: Vec<String>,
    backend: Box<dyn ChatBackend>,
) -> Result<String, Box<dyn Error>> {
    let storage = NilChatStorage {};
    let persona = Persona {
        name: "project".to_string(),
        chat_prompt: "I know how to match projects to personas".to_string(),
        file_types: vec![],
    };

    let mut chat_service = ChatService::new(backend, Box::new(storage), persona.clone(), None);

    let prompt = format!(
        "Analyze the following directory structure:\n{}\n\nChoose the most suitable persona from this list: {:?}. Just answer with one value from that list. No explanation needed.",
        dir_listing, personas
    );

    let response = chat_service
        .send_message(std::borrow::Cow::Borrowed(&prompt), false)
        .await?;
    Ok(response.trim().to_string())
}

// Function to get or prompt for OpenAI key
fn get_or_prompt_openai_key() -> Result<String, Box<dyn Error>> {
    env::var("OPENAI_KEY").or_else(|_| {
        println!("OpenAI key not found in the environment.");
        get_password_input("Please enter your OpenAI API key: ")
            .map(|key| key.trim().to_string())
            .map_err(|e| {
                eprintln!("Failed to read the OpenAI API key: {}", e);
                std::process::exit(1);
            })
    })
}

// Function to write OpenAI key to .env file
fn write_openai_key_to_env_file(openai_key: &str) -> Result<(), Box<dyn Error>> {
    if openai_key.is_empty() {
        return Err("OpenAI API key is required.".into());
    }

    let env_file_content = fs::read_to_string(".env").unwrap_or_default();

    if !env_file_content.contains("OPENAI_KEY") {
        let mut env_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(".env")?;
        writeln!(env_file, "OPENAI_KEY={}", openai_key)?;
    }

    Ok(())
}

// Function to list directory
fn get_directory_listing(path: &str) -> String {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.path().display().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

// Function to get internal personas
fn get_internal_personas() -> Vec<String> {
    get_internal_persona_configs()
        .iter()
        .map(|p| p.name.clone())
        .collect()
}

fn choose_backend_option() -> Result<AIBackend, Box<dyn Error>> {
    loop {
        let choice = get_user_input("Choose backend to use (1 for OpenAI, 2 for Ollama): ")?;
        match choice.trim() {
            "1" => return Ok(AIBackend::OpenAI),
            "2" => return Ok(AIBackend::Ollama),
            _ => println!("Invalid choice. Please enter 1 for OpenAI or 2 for Ollama."),
        }
    }
}

// Modify the write_config function to include the backend choice
fn write_config(
    recommended_persona: &str,
    model_complex: &str,
    model_fast: &str,
    ollama_url: &str,
    ollama_model: &str,
) -> Result<(), Box<dyn Error>> {
    let config_content = format!(
        r#"
default_persona = "{}"

[ai]
chat_model = "{}"
commit_model = "{}"
wish_model = "{}"

[[models]]
name = "openai_fast"
api_name = "gpt-4o-mini"
backend = "OpenAI"

[[models]]
name = "openai_complex"
api_name = "gpt-4o"
backend = "OpenAI"

[[models]]
name = "ollama_complex"
api_name = "{}"
backend = "Ollama"
url = "{}"
"#,
        recommended_persona, model_complex, model_fast, model_complex, ollama_model, ollama_url
    );

    fs::create_dir_all(".rusty")?;
    fs::write(".rusty/config.toml", config_content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_get_or_prompt_openai_key_with_env_var() {
        // Set an environment variable for OpenAI key
        env::set_var("OPENAI_KEY", "test_key");

        let key = get_or_prompt_openai_key().expect("Failed to get OpenAI key");

        assert_eq!(key, "test_key");

        // Clean up the environment variable
        env::remove_var("OPENAI_KEY");
    }

    #[test]
    fn test_write_openai_key_to_env_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let env_file_path = temp_dir.path().join(".env");

        fs::write(env_file_path.clone(), "").unwrap();

        // Set current directory to the temp dir temporarily
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();

        write_openai_key_to_env_file("test_key").expect("Failed to write API key to .env");

        let content = fs::read_to_string(env_file_path).expect("Failed to read .env file");
        assert!(content.contains("OPENAI_KEY=test_key"));

        // Reset current directory
        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_get_directory_listing() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        fs::File::create(temp_dir.path().join("file1.txt")).unwrap();
        fs::File::create(temp_dir.path().join("file2.txt")).unwrap();

        let listing = get_directory_listing(temp_dir.path().to_str().unwrap());

        assert!(listing.contains("file1.txt"));
        assert!(listing.contains("file2.txt"));
    }
}
