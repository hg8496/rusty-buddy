mod config_file;

use std::path::PathBuf;

pub use config_file::CONFIG;

pub static BASE_DIR: &str = ".rusty";

pub fn get_chat_sessions_dir() -> PathBuf {
    PathBuf::from(BASE_DIR).join("chat")
}

pub fn get_config_file() -> PathBuf {
    PathBuf::from(BASE_DIR).join("config.toml")
}
