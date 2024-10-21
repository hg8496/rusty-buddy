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
//! ```no_run
//! // Get the current application configuration
//! use rbchat::config::CONFIG;
//! let config = CONFIG.lock().unwrap();
//! println!("Using AI model: {}", config.ai.chat_model);
//! ```
//!
//! # Loading Configuration
//!
//! Configuration is loaded from a file named `config.toml` located in the `.rusty`
//! directory at runtime. If the file does not exist or cannot be read, a default
//! configuration will be created. This is done to ensure that users have a consistent
//! experience and don’t encounter unexpected crashes due to missing configurations.
//!
//! # Error Handling
//!
//! Be aware that functions within this module return `Result` types to encapsulate errors
//! that may occur during file I/O or deserialization. Always handle errors gracefully
//! to ensure a smooth user experience.
//!
//! # Example of the Configuration File
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
//! chat_timeout_secs = 30
//!
//! [[models]]
//! name = "openai_complex"
//! api_name = "gpt-4o"
//! backend = "OpenAI"
//!
//! # Add further models and personas as necessary
//! ```
//!
//! Make sure to keep this configuration updated to reflect any changes in your application
//! environment or requirements.
use crate::persona::Persona;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs;
use std::sync::Mutex;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_persona")]
    pub default_persona: String,
    #[serde(default = "default_ai")]
    pub ai: AI,
    #[serde(default = "default_personas")]
    pub personas: Vec<Persona>,
    #[serde(default = "default_models")]
    pub models: Option<Vec<Model>>,
    #[serde(default = "default_console_log_level")]
    pub console_log_level: String,
    #[serde(default = "default_file_log_level")]
    pub file_log_level: String,
}

fn default_console_log_level() -> String {
    "Warn".to_string()
}

fn default_file_log_level() -> String {
    "Info".to_string()
}
#[derive(Debug, Deserialize, Clone)]
pub struct AI {
    #[serde(default = "chat_model")]
    pub chat_model: String,

    #[serde(default = "commit_model")]
    pub commit_model: String,

    #[serde(default = "wish_model")]
    pub wish_model: String,

    #[serde(default = "embedding_model")]
    pub embedding_model: String,

    #[serde(default = "default_timeout_secs")]
    pub chat_timeout_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Model {
    pub name: String,
    pub api_name: String,
    pub url: Option<String>,
    pub backend: AIBackend,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub enum AIBackend {
    #[default]
    OpenAI,
    Ollama,
}

fn default_ai() -> AI {
    AI {
        wish_model: wish_model(),
        commit_model: commit_model(),
        chat_model: chat_model(),
        chat_timeout_secs: default_timeout_secs(),
        embedding_model: embedding_model(),
    }
}

fn default_timeout_secs() -> u64 {
    30 // Default timeout duration in seconds
}

fn default_model() -> String {
    "gpt-4o-2024-08-06".to_string()
}

fn chat_model() -> String {
    default_model()
}

fn commit_model() -> String {
    "gpt-4o-mini".to_string()
}
fn embedding_model() -> String {
    "text-embedding-3-large".to_string()
}

fn wish_model() -> String {
    default_model()
}

fn default_persona() -> String {
    "rust".to_string()
}

fn default_personas() -> Vec<Persona> {
    vec![]
}

fn default_models() -> Option<Vec<Model>> {
    None
}

// Lazy loading of global config to avoid uninitialized states
lazy_static! {
    pub static ref CONFIG: Mutex<Config> = Mutex::new(load_config().unwrap());
}

// Load configuration from file
fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = crate::config::get_config_file()?;
    match fs::read_to_string(config_path) {
        Ok(config_contents) => {
            let config: Config = toml::from_str(&config_contents)?;
            Ok(config)
        }
        Err(_) => Ok(Config {
            default_persona: default_persona(),
            ai: default_ai(),
            personas: default_personas(),
            ..Config::default()
        }),
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_persona: default_persona(),
            ai: AI {
                chat_model: "".to_string(),
                commit_model: "".to_string(),
                wish_model: "".to_string(),
                chat_timeout_secs: default_timeout_secs(),
                embedding_model: "".to_string(),
            },
            personas: vec![],
            models: None,
            console_log_level: default_console_log_level(),
            file_log_level: default_file_log_level(),
        }
    }
}
