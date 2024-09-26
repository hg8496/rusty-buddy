use crate::chat::file_storage::NilChatStorage;
use crate::chat::service::ChatService;
use crate::cli::utils::{add_to_context, get_user_input, load_files_into_context};
use crate::openai_api::openai_interface::OpenAIInterface;
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
    let openai = OpenAIInterface::new();
    let storage = NilChatStorage {};
    let mut chat_service = ChatService::new(openai, storage);

    let mut context = String::new();
    let current_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

    // Create a path for Cargo.toml in the current directory
    let cargo_toml_path = current_dir.join("Cargo.toml");

    // Check if Cargo.toml exists and read its contents
    if cargo_toml_path.exists() {
        add_to_context(&mut context, &cargo_toml_path)?;
    }

    // Load Rust files into context
    load_files_into_context(
        path.as_path(),
        vec!["rs".to_string()].as_slice(),
        &mut context,
    )?;

    // Create an introductory system message for the chat context
    let full_context = format!(
        "You are an experienced rust developer. \
        You are tasked to develop new features and adjustments on an existing project. \
        Use the supplied tools to assist the user. \
        Your favorite tool is the show_diff tool, as it gives the user the possibility to see the changes When using the create_file tool, make sure to write the complete content of the file \
        Use the following project to fulfill the wish from the user: {}",
        &context
    );
    chat_service.add_system_message(&full_context);

    // Get user input for their wish
    let user_input = get_user_input("What do you wish? ")
        .map_err(|e| format!("Failed to read user input: {}", e))?;
    let wish = format!("Users wish: {}", user_input);

    // Send the wish to the chat service and get the AI response
    let response = chat_service.send_message(&wish, use_tools).await?;

    // Print the response and statistics
    println!("AI Context Response: {}", response);

    Ok(())
}
