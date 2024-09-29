use crate::chat::file_storage::NilChatStorage;
use crate::chat::service::ChatService;
use crate::cli::utils::get_multiline_input;
use crate::config;
use crate::openai_api::openai_interface::OpenAIInterface;
use crate::persona::get_persona;
use std::error::Error;
use std::path::PathBuf;

pub async fn run_wish(directory: &str, use_tools: bool) -> Result<(), Box<dyn Error>> {
    // Check if the directory is valid
    let path = PathBuf::from(directory);
    if !path.is_dir() {
        eprintln!(
            "Error: The specified path '{}' is not a directory.",
            directory
        );
        return Ok(());
    }

    // Initialize the chat service with an OpenAI backend and NilStorage
    let config = config::CONFIG.lock().unwrap();
    let model = &config.ai.wish_model.clone();
    let default_persona = &config.default_persona.clone();
    drop(config);
    let openai = OpenAIInterface::new(String::from(model));
    let storage = NilChatStorage {};
    let persona = get_persona(default_persona.as_str()).unwrap();

    let mut chat_service = ChatService::new(
        openai,
        storage,
        persona.clone(),
        Some(directory.to_string()),
    );

    // Get user input for their wish
    let user_input = get_multiline_input("What do you wish? ")
        .map_err(|e| format!("Failed to read user input: {}", e))?;
    let wish = format!("Users wish: {}", user_input);

    // Send the wish to the chat service and get the AI response
    let response = chat_service.send_message(&wish, use_tools).await?;

    // Print the response and statistics
    println!("AI Context Response: {}", response);

    Ok(())
}
