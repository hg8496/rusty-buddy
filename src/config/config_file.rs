use crate::persona::Persona;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs;
use std::sync::Mutex;

// Configuration struct definitions
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_persona")]
    pub default_persona: String,
    #[serde(default = "default_ai")]
    pub ai: AI,
    #[serde(default = "default_personas")]
    pub personas: Vec<Persona>,
}

#[derive(Debug, Deserialize)]
pub struct AI {
    #[serde(default = "chat_model")]
    pub chat_model: String,

    #[serde(default = "commit_model")]
    pub commit_model: String,

    #[serde(default = "wish_model")]
    pub wish_model: String,
}

fn default_ai() -> AI {
    AI {
        wish_model: wish_model(),
        commit_model: commit_model(),
        chat_model: chat_model(),
    }
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

fn wish_model() -> String {
    default_model()
}

fn default_persona() -> String {
    "rust".to_string()
}

fn default_personas() -> Vec<Persona> {
    vec![]
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
        }),
    }
}
