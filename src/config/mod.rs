mod config_file;

use std::env;
use std::path::PathBuf;

pub use config_file::CONFIG;

static BASE_DIR: &str = ".rusty";

// Retrieve the directory for chat sessions
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
    use tempdir::TempDir;

    #[test]
    fn test_get_config_file_success_from_temp_directory() {
        let temp_dir = TempDir::new("example").expect("Failed to create temp dir");

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
        let temp_dir =
            TempDir::new("test_get_config_file_not_found").expect("Failed to create temp dir");

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
