use crate::persona::Persona;
use serde::Deserialize;

use crate::config::get_config_file;
use lazy_static::lazy_static;
use std::fs;
use std::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct Config {
    //    openai_api_key: String,
    #[serde(default = "default_persona")]
    pub default_persona: String,

    #[serde(default = "default_ai")]
    pub ai: AI,

    #[serde(default = "personas")]
    pub personas: Vec<Persona>, // Array of Product structs
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
    default_model()
}

fn wish_model() -> String {
    default_model()
}

fn default_persona() -> String {
    "rust".to_string()
}

fn personas() -> Vec<Persona> {
    vec![]
}

lazy_static! {
    pub static ref CONFIG: Mutex<Config> = Mutex::new(load_config().unwrap());
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    match fs::read_to_string(get_config_file()) {
        Ok(config_contents) => {
            let config: Config = toml::de::from_str(&config_contents)?;
            Ok(config)
        }
        Err(_e) => Ok(toml::de::from_str("")?),
    }
}
