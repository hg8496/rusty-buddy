use crate::chat::file_storage::NilChatStorage;
use crate::chat::service::ChatService;
use crate::cli::editor::get_password_input;
use crate::openai_api::openai_interface::OpenAIInterface;
use crate::persona::{get_internal_persona_configs, Persona};
use dotenvy::dotenv;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

pub async fn run_init_command() -> Result<(), Box<dyn Error>> {
    // Load existing environment variables
    dotenv().ok();

    // Check if OPENAI_KEY is set
    let openai_key = env::var("OPENAI_KEY").unwrap_or_else(|_| {
        // Prompt user for OpenAI key if not present
        println!("OpenAI key not found in the environment.");

        match get_password_input("Please enter your OpenAI API key: ") {
            Ok(key) => key.trim().to_string(),
            Err(e) => {
                eprintln!("Failed to read the OpenAI API key: {}", e);
                std::process::exit(1);
            }
        }
    });
    // Ensure OPENAI_KEY is set and write to .env file if necessary
    ensure_openai_key_in_env(&openai_key)?;

    // Fetch recursive directory listing
    let files = get_recursive_dir_listing(".");
    let persona_configs = get_internal_persona_configs();
    let personas = persona_configs.iter().map(|p| p.name.as_str()).collect();

    // Generate persona recommendation using the ChatService
    let recommended_persona = recommend_persona(files, personas).await?;

    // Write config.toml based on recommended persona
    write_config(&recommended_persona)?;

    println!(
        "Configuration successfully initialized with persona: {}",
        recommended_persona
    );

    Ok(())
}

fn ensure_openai_key_in_env(openai_key: &str) -> Result<(), Box<dyn Error>> {
    if openai_key.is_empty() {
        return Err("OpenAI API key is required.".into());
    }

    if Path::new(".env").exists() {
        let content = fs::read_to_string(".env")?;
        if !content.contains("OPENAI_KEY") {
            let mut env_file = fs::OpenOptions::new().append(true).open(".env")?;
            writeln!(env_file, "OPENAI_KEY={}", openai_key)?;
        }
    } else {
        fs::write(".env", format!("OPENAI_KEY={}", openai_key))?;
    }

    Ok(())
}

fn get_recursive_dir_listing(path: &str) -> String {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|entry| entry.path().display().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

async fn recommend_persona(
    dir_listing: String,
    personas: Vec<&str>,
) -> Result<String, Box<dyn Error>> {
    let client = OpenAIInterface::new("gpt-4o-mini".to_string());
    let storage = NilChatStorage {};
    let persona = Persona {
        name: "project".to_string(),
        chat_prompt: "I know how to match projects to personas".to_string(),
        file_types: vec![],
    };

    let mut chat_service = ChatService::new(client, storage, persona.clone(), None);

    let prompt = format!(
        "Analyze the following directory structure:\n{}\n\nChoose the most suitable persona from this list: {:?}. Just answer with one value from that list. No explanation needed.",
        dir_listing, personas
    );

    let response = chat_service.send_message(&prompt, false).await?;
    Ok(response.trim().to_string())
}

fn write_config(recommended_persona: &str) -> Result<(), Box<dyn Error>> {
    let config_content = format!(
        "default_persona = \"{}\"\n[ai]\nchat_model = \"gpt-4o-2024-08-06\"\ncommit_model = \"gpt-4o-mini\"\n",
        recommended_persona
    );
    fs::create_dir_all(".rusty")?;
    fs::write(".rusty/config.toml", config_content)?;
    Ok(())
}
