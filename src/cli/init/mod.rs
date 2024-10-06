use crate::chat::file_storage::NilChatStorage;
use crate::chat::service::ChatService;
use crate::cli::editor::get_password_input;
use crate::persona::{get_internal_persona_configs, Persona};
use crate::provider::openai::openai_interface::OpenAIInterface;
use dotenvy::dotenv;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use walkdir::WalkDir;

// This function represents the entry point of the init command
pub async fn run_init_command() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let openai_key = get_or_prompt_openai_key()?;
    write_openai_key_to_env_file(&openai_key)?;

    let files = get_directory_listing(".");
    let personas = get_internal_personas();

    let recommended_persona = recommend_persona(files, personas).await?;
    write_config(&recommended_persona)?;

    println!(
        "Configuration successfully initialized with persona: {}",
        recommended_persona
    );

    Ok(())
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

// Function to recommend a persona
async fn recommend_persona(
    dir_listing: String,
    personas: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    let client = OpenAIInterface::new("gpt-4o-mini".to_string());
    let storage = NilChatStorage {};
    let persona = Persona {
        name: "project".to_string(),
        chat_prompt: "I know how to match projects to personas".to_string(),
        file_types: vec![],
    };

    let mut chat_service =
        ChatService::new(Box::new(client), Box::new(storage), persona.clone(), None);

    let prompt = format!(
        "Analyze the following directory structure:\n{}\n\nChoose the most suitable persona from this list: {:?}. Just answer with one value from that list. No explanation needed.",
        dir_listing, personas
    );

    let response = chat_service.send_message(&prompt, false).await?;
    Ok(response.trim().to_string())
}

// Function to write configuration
fn write_config(recommended_persona: &str) -> Result<(), Box<dyn Error>> {
    let config_content = format!(
        r#"
default_persona = "{}"

[ai]
chat_model = "openai_complex"
commit_model = "openai_fast"
wish_model = "openai_complex"

[[models]]
name = "openai_fast"
api_name = "gpt-4o-mini"
backend = "OpenAI"

[[models]]
name = "openai_complex"
api_name = "gpt-4o-2024-08-06"
backend = "OpenAI"

[[models]]
name = "ollama_complex"
api_name = "llama3.2"
backend = "Ollama"
url = "http://localhost:11434"
"#,
        recommended_persona
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
