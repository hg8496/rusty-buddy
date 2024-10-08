//! This module provides configuration management for the application, including the
//! default settings for AI models, personas, and other related configurations.
//!
//! The `Config` struct holds the overall configuration, while the `AI` struct contains
//! specific settings for the AI models including timeout durations and model identifiers.
//!
//! Default values for various settings are provided through functions that utilize the
//! `serde` library for deserialization of configuration files in TOML format. The
//! application lazily loads the configuration using a `Mutex` to ensure thread safety
//! and prevent uninitialized states. The `CONFIG` variable provides a globally accessible
//! configuration instance which can be accessed throughout the application.
//!
//! # Configuration Structure
//!
//! The `Config` structure contains:
//! - `default_persona`: The default persona to use in chat sessions.
//! - `ai`: The AI settings, containing the models used for various functionalities.
//! - `personas`: A list of defined personas that can be utilized for tailored interactions.
//! - `models`: Additional configurations for AI models, including their identifiers and APIs.
//!
//! Here’s an example of how you can utilize this module:
//!
//! ```rust
//! // Get the current application configuration
//! let config = CONFIG.lock().unwrap();
//! println!("Using AI model: {}", config.ai.chat_model);
//! ```
//!
//! # Loading Configuration
//!
//! Configuration is loaded from a file named `config.toml` located in the `.rusty`
//! directory at runtime. If the file does not exist or cannot be read, a default
//! configuration will be created to ensure that users have a consistent experience
//! and don’t encounter unexpected crashes due to missing configurations.
//!
//! # Error Handling
//!
//! Be aware that functions within this module return `Result` types to encapsulate errors
//! that may occur during file I/O or deserialization. Always handle errors gracefully
//! to ensure a smooth user experience.
//!
//! # Examples of the Configuration File
//!
//! Below is an example of how the `config.toml` might be structured:
//!
//! ```toml
//! default_persona = "rust"
//!
//! [ai]
//! chat_model = "openai_complex"
//! commit_model = "openai_fast"
//! wish_model = "openai_complex"
//!
//! [[models]]
//! name = "openai_fast"
//! api_name = "gpt-4o-mini"
//! backend = "OpenAI"
//!
//! [[models]]
//! name = "openai_complex"
//! api_name = "gpt-4o-2024-08-06"
//! backend = "OpenAI"
//!
//! # Add further models and personas as necessary
//! ```
//! Functions to locate the configuration file for the application.
//! The configuration file is expected to be located at '.rusty/config.toml'
//! relative to the current working directory or any of its parent directories.
mod config_file;

use std::env;
use std::path::PathBuf;

pub use config_file::AIBackend;
pub use config_file::Config;
pub use config_file::CONFIG;

static BASE_DIR: &str = ".rusty";

pub fn get_chat_sessions_dir() -> Result<PathBuf, String> {
    let config_file = get_config_file()?;
    let config_dir = config_file.parent().expect("Expected a parent directory");
    Ok(config_dir.join("chat"))
}

pub fn get_config_file() -> Result<PathBuf, String> {
    get_config_file_from_dir(
        env::current_dir().map_err(|e| format!("Failed to get the current directory: {}", e))?,
    )
}

pub fn get_config_file_from_dir(mut current_dir: PathBuf) -> Result<PathBuf, String> {
    loop {
        let config_file_path = current_dir.join(BASE_DIR).join("config.toml");
        if config_file_path.exists() {
            return Ok(config_file_path);
        }
        if !current_dir.pop() {
            return Err(
                "No '.rusty/config.toml' found in current or any parent directory.".to_string(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_get_config_file_success_from_temp_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create the .rusty directory within the temp directory
        let rusty_dir = temp_dir.path().join(".rusty");
        fs::create_dir_all(&rusty_dir).expect("Failed to create .rusty dir");

        // Create a config.toml file within that directory
        let config_path = rusty_dir.join("config.toml");
        fs::File::create(&config_path).expect("Failed to create config.toml");

        // Change current working directory to the temporary directory
        //env::set_current_dir(&temp_dir).expect("Failed to set current directory");

        // Run the function you're testing
        let result =
            get_config_file_from_dir(temp_dir.into_path()).unwrap_or(PathBuf::from("/wrong_path"));

        // Assert the expected outcome
        assert!(result.ends_with(config_path));
    }

    #[test]
    fn test_get_config_file_not_found() {
        // Set up a temporary directory without a config file
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Confirm that the path does not exist
        let config_file = temp_dir.path().join(".rusty").join("config.toml");
        assert!(!config_file.exists());

        // Change current working directory to the temporary directory

        // Call the function and assert the expected error message
        let result = get_config_file_from_dir(temp_dir.into_path());
        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some("No '.rusty/config.toml' found in current or any parent directory.".to_string())
        );
    }
}
