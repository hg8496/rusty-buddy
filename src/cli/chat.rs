use crate::chat::file_storage::DirectoryChatStorage;
use crate::chat::interface::ChatStorage;
use crate::chat::service::ChatService;
use crate::cli::spinner::{start_spinner, stop_spinner};
use crate::cli::style::configure_mad_skin;
use crate::cli::utils::{get_user_input, load_files_into_context};
use crate::openai_api::openai_interface::OpenAIInterface;
use std::error::Error;
use std::path::Path;

const CHAT_DIR: &str = ".rusty/chats";

pub async fn run_chat(
    start_new: bool,
    continue_last: bool,
    load_name: Option<String>,
    directory: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let openai = OpenAIInterface::new();
    let storage = DirectoryChatStorage::new(CHAT_DIR);
    let mut chat_service = ChatService::new(openai, storage);
    chat_service.add_system_message("You are an seasoned rust developer. Your are helping a dear colleage developing new features and anwering questions.");
    let mut start_session = start_new;
    if continue_last {
        if let Some(last_session) = get_last_session_name()? {
            println!("Continuing the last session: {}", last_session);
            chat_service.load_history(&last_session)?;
        } else {
            println!("No previous session found. Starting a new chat.");
            start_session = true;
        }
    } else if let Some(session_name) = load_name {
        println!("Loading session: {}", session_name);
        chat_service.load_history(&session_name)?;
    } else {
        println!("No session option provided. Defaulting to a new chat.");
        start_session = true;
    }

    if start_session {
        println!("Starting new Chat.");
        if let Some(dir) = directory {
            println!("Loading context.");
            let path = Path::new(&dir);
            let mut context = String::from("Use the following Context to assist the user.\n");
            load_files_into_context(path, "rs", &mut context)?;
            chat_service.add_system_message(&context.as_str());
        }
    }

    // Create a MadSkin to style the terminal output
    let skin = configure_mad_skin();

    loop {
        let user_input = get_user_input("User (use Ctrl+D to submit): ")?;
        let trimmed_input = user_input.trim();

        if trimmed_input == "exit" || trimmed_input.is_empty() {
            let save_name = get_user_input(
                "Enter a name to save this session (or press Enter to skip saving): ",
            )?;
            if !save_name.trim().is_empty() {
                chat_service.save_history(&save_name.trim())?;
            }
            break;
        }

        // Start the spinner
        let spinner = start_spinner();

        // Send the message to OpenAI using ChatService
        let response = chat_service.send_message(trimmed_input, false).await?;

        // Stop the spinner
        stop_spinner(spinner);

        // Use the skin to print the AI's response with styling
        skin.print_text("-----\nAI:");
        skin.print_text(&response);
        chat_service.print_statistics();
    }

    println!("You have exited the chat.");
    Ok(())
}

// Helper function to get the name of the last session
fn get_last_session_name() -> Result<Option<String>, Box<dyn Error>> {
    let sessions = DirectoryChatStorage::new(CHAT_DIR).list_sessions()?;
    Ok(sessions.last().cloned())
}
