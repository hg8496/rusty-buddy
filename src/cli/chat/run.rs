use crate::chat::command_registry::CommandRegistry;
use crate::chat::commands::initialize_commands;
use crate::chat::file_storage::DirectoryChatStorage;
use crate::chat::interface::ChatStorage;
use crate::chat::service::ChatService;
use crate::cli::spinner::{start_spinner, stop_spinner};
use crate::cli::style::configure_mad_skin;
use crate::cli::utils::{get_multiline_input, get_user_input};
use crate::config;
use crate::config::get_chat_sessions_dir;
use crate::openai_api::openai_interface::OpenAIInterface;
use crate::persona::get_persona;
use atty::Stream;
use std::error::Error;
use std::io::{self, Read};

pub async fn run_chat(
    start_new: bool,
    continue_last: bool,
    load_name: Option<String>,
    directory: Option<String>,
    persona_name: Option<String>,
    one_shot: bool,
    input_message: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let (model, default_persona) = get_config();
    let openai = OpenAIInterface::new(String::from(&model));
    let storage = DirectoryChatStorage::new(config::get_chat_sessions_dir());
    let mut command_registry = CommandRegistry::new();

    // Register commands
    initialize_commands(&mut command_registry);

    // Determine the persona to use
    let persona = match persona_name {
        Some(name) => match get_persona(&name) {
            Some(p) => p,
            None => {
                eprintln!("Specified persona not found. Using default.");
                get_persona(default_persona.as_str()).unwrap()
            }
        },
        None => get_persona(default_persona.as_str()).unwrap(),
    };
    let mut chat_service = ChatService::new(openai, storage, persona.clone(), directory);

    let mut start_session = start_new;
    if continue_last {
        if let Some(last_session) = get_last_session_name()? {
            eprintln!("Continuing the last session: {}", last_session);
            chat_service.load_history(&last_session)?;
        } else {
            eprintln!("No previous session found. Starting a new chat.");
            start_session = true;
        }
    } else if let Some(session_name) = load_name {
        eprintln!("Loading session: {}", session_name);
        chat_service.load_history(&session_name)?;
    } else {
        eprintln!("No session option provided. Defaulting to a new chat.");
        start_session = true;
    }

    if start_session {
        chat_service.setup_context();
    }
    let skin = configure_mad_skin();

    if one_shot {
        // Initialize a buffer to read data from stdin
        let mut buffer = String::new();

        // Check if there's a message passed via command line options
        let user_input = if let Some(message) = input_message {
            message
        } else if !atty::is(Stream::Stdin) {
            // Read from stdin if it's not a terminal (i.e., piped input)
            io::stdin().read_to_string(&mut buffer)?;
            buffer.trim().to_string() // Trim whitespace
        } else {
            get_multiline_input("Your message (end with Ctrl+D): ")?
        };

        // Send the message to OpenAI using ChatService
        if !user_input.trim().is_empty() {
            let spinner = start_spinner();

            let response = chat_service.send_message(user_input.trim(), false).await?;
            stop_spinner(spinner);

            skin.print_text(
                format!("---\n# AI Persona:{} Model: {}\n", persona.name, model).as_str(),
            );
            skin.print_text(&response);
            chat_service.print_statistics();
        } else {
            println!("No input provided.");
        }
        return Ok(());
    }
    // Interactive chat loop
    loop {
        let user_input = get_multiline_input("User (use Ctrl+D to submit): ")?;
        let trimmed_input = user_input.trim();

        if trimmed_input.starts_with('/') {
            let mut parts = trimmed_input.split_whitespace();
            let command_name = parts.next().unwrap_or("");
            let args: Vec<&str> = parts.collect();
            if let Err(e) = command_registry.execute_command(command_name, &args, &mut chat_service)
            {
                eprintln!("Unknown command '{}', error: {}", command_name, e);
            }
            continue;
        }
        if trimmed_input == "exit" || trimmed_input.is_empty() {
            let save_name = get_user_input(
                "Enter a name to save this session (or press Enter to skip saving): ",
            )?;
            if !save_name.trim().is_empty() {
                chat_service.save_history(save_name.trim())?;
            }
            break;
        }

        let spinner = start_spinner();

        let response = chat_service.send_message(trimmed_input, false).await?;

        stop_spinner(spinner);

        skin.print_text(format!("---\n# AI Persona:{} Model: {}\n", persona.name, model).as_str());
        skin.print_text(&response);
        chat_service.print_statistics();
    }

    println!("You have exited the chat.");
    Ok(())
}

fn get_config() -> (String, String) {
    let config = config::CONFIG.lock().unwrap();
    let model = config.ai.chat_model.clone();
    let default_persona = config.default_persona.clone();
    (model, default_persona)
}

// Helper function to get the name of the last session
fn get_last_session_name() -> Result<Option<String>, Box<dyn Error>> {
    let sessions = DirectoryChatStorage::new(get_chat_sessions_dir()).list_sessions()?;
    Ok(sessions.last().cloned())
}
