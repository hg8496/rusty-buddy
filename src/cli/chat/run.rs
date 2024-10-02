use crate::chat::command_registry::CommandRegistry;
use crate::chat::commands::initialize_commands;
use crate::chat::file_storage::DirectoryChatStorage;
use crate::chat::interface::{ChatStorage, MessageRole};
use crate::chat::service::ChatService;
use crate::cli::chat::ChatArgs;
use crate::cli::spinner::{start_spinner, stop_spinner};
use crate::cli::style::configure_mad_skin;
use crate::cli::utils::{get_multiline_input, get_user_input};
use crate::config;
use crate::config::get_chat_sessions_dir;
use crate::openai_api::openai_interface::OpenAIInterface;
use crate::persona::{get_persona, Persona};
use atty::Stream;
use std::error::Error;
use std::io::{self, Read};

pub async fn run_chat(args: ChatArgs) -> Result<(), Box<dyn Error>> {
    let (model, default_persona) = get_config();
    let openai = OpenAIInterface::new(model.clone());
    let storage = DirectoryChatStorage::new(get_chat_sessions_dir()?);
    let command_registry = initialize_command_registry();

    let persona = resolve_persona(&args.persona, &default_persona)?;
    let mut chat_service = ChatService::new(openai, storage, persona.clone(), args.directory);

    handle_session(&mut chat_service, args.new, args.continue_last, &args.load)?;

    if args.one_shot {
        return handle_one_shot_mode(chat_service, args.message, model, persona).await;
    }

    if (args.continue_last || args.load.is_some()) && !args.silence {
        print_loaded_messages(&chat_service);
    }

    start_interactive_chat(chat_service, command_registry, model, persona).await
}

fn initialize_command_registry() -> CommandRegistry<'static> {
    let mut command_registry = CommandRegistry::new();
    initialize_commands(&mut command_registry);
    command_registry
}

fn resolve_persona(
    persona_name: &Option<String>,
    default_persona: &str,
) -> Result<Persona, Box<dyn Error>> {
    match persona_name {
        Some(name) => {
            get_persona(name).ok_or_else(|| "Specified persona not found. Using default.".into())
        }
        None => Ok(get_persona(default_persona).unwrap()),
    }
}

fn handle_session(
    chat_service: &mut ChatService<OpenAIInterface, DirectoryChatStorage>,
    start_new: bool,
    continue_last: bool,
    load_name: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    if start_new {
        chat_service.setup_context();
    } else {
        handle_session_loading(chat_service, continue_last, load_name)?;
    }
    Ok(())
}

fn handle_session_loading(
    chat_service: &mut ChatService<OpenAIInterface, DirectoryChatStorage>,
    continue_last: bool,
    load_name: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    if continue_last {
        if let Some(last_session) = get_last_session_name()? {
            eprintln!("Continuing the last session: {}", last_session);
            chat_service.load_history(&last_session)?;
        } else {
            eprintln!("No previous session found. Starting a new chat.");
            chat_service.setup_context();
        }
    } else if let Some(session_name) = load_name {
        eprintln!("Loading session: {}", session_name);
        chat_service.load_history(session_name)?;
    } else {
        chat_service.setup_context();
    }
    Ok(())
}

// Function to print loaded messages
fn print_loaded_messages(chat_service: &ChatService<OpenAIInterface, DirectoryChatStorage>) {
    let skin = configure_mad_skin();
    chat_service.process_messages(|msg| match msg.role {
        MessageRole::User => skin.print_text(&format!("# User\n{}\n---\n", msg.content)),
        MessageRole::Assistant => skin.print_text(&format!("# Assistant\n{}\n---\n", msg.content)),
        _ => {}
    });
}

async fn handle_one_shot_mode(
    mut chat_service: ChatService<OpenAIInterface, DirectoryChatStorage>,
    input_message: Option<String>,
    model: String,
    persona: Persona,
) -> Result<(), Box<dyn Error>> {
    let user_input = get_user_input_from_option_or_stdin(input_message)?;
    if user_input.trim().is_empty() {
        println!("No input provided.");
        return Ok(());
    }

    send_and_display_response(&mut chat_service, &user_input, &model, &persona).await
}

async fn start_interactive_chat(
    mut chat_service: ChatService<OpenAIInterface, DirectoryChatStorage>,
    mut command_registry: CommandRegistry<'_>,
    model: String,
    persona: Persona,
) -> Result<(), Box<dyn Error>> {
    loop {
        let user_input = get_multiline_input("User (use Ctrl+D to submit): ")?;
        let trimmed_input = user_input.trim();

        if trimmed_input.starts_with('/') {
            handle_command(&mut command_registry, trimmed_input, &mut chat_service);
            continue;
        }

        if trimmed_input == "exit" || trimmed_input.is_empty() {
            save_session_if_requested(&mut chat_service)?;
            break;
        }

        send_and_display_response(&mut chat_service, trimmed_input, &model, &persona).await?;
    }

    println!("You have exited the chat.");
    Ok(())
}

fn handle_command(
    command_registry: &mut CommandRegistry,
    trimmed_input: &str,
    chat_service: &mut ChatService<OpenAIInterface, DirectoryChatStorage>,
) {
    let mut parts = trimmed_input.split_whitespace();
    let command_name = parts.next().unwrap_or("");
    let args: Vec<&str> = parts.collect();

    if let Err(e) = command_registry.execute_command(command_name, &args, chat_service) {
        eprintln!("Unknown command '{}', error: {}", command_name, e);
    }
}

fn save_session_if_requested(
    chat_service: &mut ChatService<OpenAIInterface, DirectoryChatStorage>,
) -> Result<(), Box<dyn Error>> {
    let save_name =
        get_user_input("Enter a name to save this session (or press Enter to skip saving): ")?;
    if !save_name.trim().is_empty() {
        chat_service.save_history(save_name.trim())?;
    }
    Ok(())
}

async fn send_and_display_response(
    chat_service: &mut ChatService<OpenAIInterface, DirectoryChatStorage>,
    user_input: &str,
    model: &str,
    persona: &Persona,
) -> Result<(), Box<dyn Error>> {
    let spinner = start_spinner();

    let response = chat_service.send_message(user_input.trim(), false).await?;
    stop_spinner(spinner);

    let skin = configure_mad_skin();
    skin.print_text(&format!(
        "---\n# AI Persona:{} Model: {}\n",
        persona.name, model
    ));
    skin.print_text(&response);
    chat_service.print_statistics();

    Ok(())
}

fn get_user_input_from_option_or_stdin(
    input_message: Option<String>,
) -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    if let Some(message) = input_message {
        Ok(message)
    } else if !atty::is(Stream::Stdin) {
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer.trim().to_string())
    } else {
        get_multiline_input("Your message (end with Ctrl+D): ")
    }
}

fn get_config() -> (String, String) {
    let config = config::CONFIG.lock().unwrap();
    (config.ai.chat_model.clone(), config.default_persona.clone())
}

fn get_last_session_name() -> Result<Option<String>, Box<dyn Error>> {
    let sessions = DirectoryChatStorage::new(get_chat_sessions_dir()?).list_sessions()?;
    Ok(sessions.last().cloned())
}
