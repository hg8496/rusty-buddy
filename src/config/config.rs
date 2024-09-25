use crate::persona::Persona;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    //    openai_api_key: String,
    #[serde(default = "default_persona")]
    pub default_persona: String,
    #[serde(default = "personas")]
    pub personas: Vec<Persona>, // Array of Product structs
}

use crate::config::get_config_file;
use lazy_static::lazy_static;
use std::fs;
use std::sync::Mutex;

lazy_static! {
    pub static ref CONFIG: Mutex<Config> = Mutex::new(load_config().unwrap());
}

fn default_persona() -> String {
    "rust".to_string()
}

fn personas() -> Vec<Persona> {
    vec![]
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
