use std::path::PathBuf;

pub mod config;

pub use config::CONFIG;

pub static BASE_DIR: &str = ".rusty";

pub fn get_chat_sessions_dir() -> PathBuf {
    PathBuf::from(BASE_DIR).join("chat")
}

pub fn get_config_file() -> PathBuf {
    PathBuf::from(BASE_DIR).join("config.toml")
}
