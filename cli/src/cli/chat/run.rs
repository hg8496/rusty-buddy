//! This module provides functionality for managing chat sessions, initializing necessary components,
//! and handling user interactions within the Rusty Buddy application. The primary function, `run_chat`,
//! is responsible for orchestrating chat interactions, managing session states, and resolving
//! user-defined personas to ensure an enriched chat experience with the AI assistant.
//!
//! ## Key Responsibilities
//!
//! - **Session Management**: Initializes and tracks chat sessions, including the ability to continue or load previous sessions.
//! - **Command Handling**: Interfaces with registered commands, allowing for interaction via chat commands.
//! - **User Input Handling**: Captures and processes user messages, providing responses derived from the AI backend.
//! - **Context Setup**: Facilitates loading of relevant files and messages to provide a strengthened context for the chat session.
//!
//! ## Example Usage
//!
//! Here's how to initiate a chat session and engage with the AI assistant:
//!
//! ```rust
//! use crate::cli::chat::{ChatArgs, run};
//!
//! let args = ChatArgs {
//!     new: true,
//!     continue_last: false,
//!     load: None,
//!     directory: Some(String::from("./src")),
//!     persona: Some(String::from("rust")),
//!     one_shot: false,
//!     message: None,
//!     silence: false,
//! };
//!
//! run(args).await.unwrap();
//! ```
//!
//! ## Functions
//!
//! - `run_chat`: Initializes a `ChatService`, manages the flow of the chat session, and processes user inputs,
//!   invoking the appropriate logic based on command arguments and context.
//!
//! ## Error Handling
//!
//! Each function is designed to return a `Result` type, encapsulating any errors that may arise during execution.
//! Careful attention is given to ensure that users receive meaningful error messages when unexpected conditions occur.
//! Implementations should handle potential failures gracefully, maintaining a seamless user experience.

use crate::cli::chat::commands::initialize_cli_commands;
use crate::cli::chat::ChatArgs;
use crate::cli::editor::{get_multiline_input, get_user_input};
use crate::cli::spinner::{start_spinner, stop_spinner};
use crate::cli::style::configure_mad_skin;
use atty::Stream;
use chrono::{DateTime, Local, Utc};
use log::error;
use rbchat::chat::command_registry::CommandRegistry;
use rbchat::chat::commands::initialize_commands;
use rbchat::chat::file_storage::DirectoryChatStorage;
use rbchat::chat::interface::{ChatStorage, MessageInfo, MessageRole};
use rbchat::chat::service::ChatService;
use rbchat::config;
use rbchat::config::{get_chat_sessions_dir, Config};
use rbchat::knowledge::{KnowledgeStore, StoreBuilder};
use rbchat::persona::{resolve_persona, Persona};
use std::borrow::Cow;
use std::error::Error;
use std::io::{self, Read};
use std::path::PathBuf;
use std::sync::Arc;

struct Services {
    chat_service: ChatService,
    knowledge_store: Arc<dyn KnowledgeStore>,
}

/// Runs the chat application, initializing the necessary components,
/// handling command line arguments, and starting either an interactive
/// chat session or a one-shot message response based on the provided
/// arguments. It manages chat sessions and persona resolution.
pub async fn run_chat(args: ChatArgs) -> Result<(), Box<dyn Error>> {
    let config = get_config();
    let storage = DirectoryChatStorage::new(get_chat_sessions_dir()?);
    let command_registry = initialize_command_registry();

    let persona = resolve_persona(&args.persona, config.default_persona.as_str())?;
    let model_name = args
        .model
        .as_deref()
        .unwrap_or(config.ai.chat_model.as_str());

    let mut services = Services {
        chat_service: ChatService::builder()
            .model_name(model_name)
            .storage(Box::new(storage))
            .persona(persona.clone())
            .directory(args.directory)
            .build()?,
        knowledge_store: StoreBuilder::new().build().await?,
    };

    handle_session(
        &mut services.chat_service,
        args.new,
        args.continue_last,
        &args.load,
    )?;

    if args.one_shot.is_some() {
        let message = args.one_shot.as_ref().unwrap();
        return handle_one_shot_mode(
            services,
            message.clone(),
            model_name,
            persona,
            args.knowledge,
            &args.image,
        )
        .await;
    }

    if (args.continue_last || args.load.is_some()) && !args.silence {
        print_loaded_messages(&services.chat_service);
    }

    start_interactive_chat(
        services,
        command_registry,
        model_name,
        persona,
        args.knowledge,
        &args.image,
    )
    .await
}

fn initialize_command_registry() -> CommandRegistry {
    let mut command_registry = CommandRegistry::new();
    initialize_commands(&mut command_registry);
    initialize_cli_commands(&mut command_registry);
    command_registry
}

fn handle_session(
    chat_service: &mut ChatService,
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
    chat_service: &mut ChatService,
    continue_last: bool,
    load_name: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    if continue_last {
        match get_last_session_name()? {
            Some(last_session) => {
                eprintln!("Continuing the last session: {}", last_session);
                chat_service.load_history(&last_session)?;
            }
            _ => {
                eprintln!("No previous session found. Starting a new chat.");
                chat_service.setup_context();
            }
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
fn print_loaded_messages(chat_service: &ChatService) {
    let is_terminal = is_output_to_terminal();

    chat_service.process_messages(|msg| {
        match msg.role {
            MessageRole::User => {
                let timestamp = msg
                    .info
                    .as_ref()
                    .and_then(|info| {
                        if let MessageInfo::UserInfo { timestamp, .. } = info {
                            Some(timestamp) // Use a reference to avoid cloning
                        } else {
                            None
                        }
                    })
                    .unwrap_or(&DateTime::<Utc>::MIN_UTC); // Default to an empty string if model is None

                print_with_optional_formatting(
                    "User",
                    "",
                    timestamp,
                    msg.content.as_str(),
                    is_terminal,
                );
            }
            MessageRole::Assistant => {
                // Use `and_then` to directly extract the model if it exists.
                let (model, persona, timestamp) = msg
                    .info
                    .as_ref()
                    .and_then(|info| {
                        if let MessageInfo::AssistantInfo {
                            model,
                            persona_name,
                            timestamp,
                            ..
                        } = info
                        {
                            Some((model.as_str(), persona_name.as_str(), timestamp))
                        // Use a reference to avoid cloning
                        } else {
                            None
                        }
                    })
                    .unwrap_or(("", "", &DateTime::<Utc>::MIN_UTC)); // Default to an empty string if model is None

                print_with_optional_formatting(
                    persona,
                    model,
                    timestamp,
                    msg.content.as_str(),
                    is_terminal,
                );
            }
            _ => {}
        }
    });
}

async fn handle_one_shot_mode(
    mut chat_service: Services,
    input_message: Option<String>,
    model: &str,
    persona: Persona,
    knowledge: Option<Option<usize>>,
    image_path: &Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    let user_input: Cow<'_, str> = Cow::Owned(get_user_input_from_option_or_stdin(input_message)?);
    if user_input.trim().is_empty() {
        error!("No input provided.");
        return Ok(());
    }

    let result = send_and_display_response(
        &mut chat_service,
        user_input,
        model,
        &persona,
        knowledge,
        image_path,
    )
    .await;
    match result {
        Ok(_) => result,
        Err(e) => {
            error!("Error sending message: {}", e);
            Err(e)
        }
    }
}

async fn start_interactive_chat(
    mut chat_service: Services,
    mut command_registry: CommandRegistry,
    model: &str,
    persona: Persona,
    knowledge: Option<Option<usize>>,
    image_path: &Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    let mut ipath = image_path;
    loop {
        let user_input: Cow<'_, str> = Cow::Owned(get_multiline_input(
            "User (use Ctrl+D to submit): ",
            command_registry.get_completions(),
        )?);
        let trimmed_input = user_input.trim();

        if trimmed_input.starts_with('/') {
            handle_command(
                &mut command_registry,
                trimmed_input,
                &mut chat_service.chat_service,
            );
            continue;
        }

        if trimmed_input == "exit" || trimmed_input.is_empty() {
            save_session_if_requested(&mut chat_service.chat_service).err();
            // Print exit message only if it's a terminal output
            if is_output_to_terminal() {
                println!("You have exited the chat.");
            }
            break;
        }

        let result = send_and_display_response(
            &mut chat_service,
            Cow::Borrowed(trimmed_input),
            model,
            &persona,
            knowledge,
            ipath,
        )
        .await;
        ipath = &None;
        match result {
            Ok(_) => continue,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
    }

    Ok(())
}

fn handle_command(
    command_registry: &mut CommandRegistry,
    trimmed_input: &str,
    chat_service: &mut ChatService,
) {
    let mut parts = trimmed_input.split_whitespace();
    let command_name = parts.next().unwrap_or("");
    let args: Vec<&str> = parts.collect();

    if let Err(e) = command_registry.execute_command(command_name, &args, chat_service) {
        eprintln!("Unknown command '{}', error: {}", command_name, e);
    }
}

fn save_session_if_requested(chat_service: &mut ChatService) -> Result<(), Box<dyn Error>> {
    let save_name =
        get_user_input("Enter a name to save this session (or press Enter to skip saving): ")?;
    if !save_name.trim().is_empty() {
        chat_service.save_history(save_name.trim())?;
    }
    Ok(())
}

fn print_with_optional_formatting(
    persona: &str,
    model: &str,
    timestamp: &DateTime<Utc>,
    context: &str,
    use_formatting: bool,
) {
    if use_formatting {
        let local_time: DateTime<Local> = timestamp.with_timezone(&Local);
        let time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
        let skin = configure_mad_skin(); // Assuming MadSkin is a struct that should be configured

        // Prepare the output string
        let mut output = String::from("---\n");

        if persona == "User" {
            output.push_str(&format!("# User input @{}:\n{}", time, context));
        } else {
            output.push_str(&format!(
                "# AI Persona: {} Model: {} @{}\n{}",
                persona, model, time, context
            ));
        }

        skin.print_text(&output); // Assuming print_text works with `&str`
    } else {
        println!("{}", context);
    }
}

fn is_output_to_terminal() -> bool {
    atty::is(Stream::Stdout)
}

async fn send_and_display_response(
    services: &mut Services,
    user_input: Cow<'_, str>,
    model: &str,
    persona: &Persona,
    knowledge: Option<Option<usize>>,
    image_path: &Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    let is_terminal = is_output_to_terminal();
    let spinner = if is_terminal {
        Some(start_spinner())
    } else {
        None
    };
    if knowledge.is_some() {
        let knowledge = services
            .knowledge_store
            .query_knowledge(user_input.clone(), knowledge.unwrap().unwrap_or(10))
            .await?;
        services.chat_service.add_knowledge(knowledge).await?;
    }
    let result = services
        .chat_service
        .send_message(user_input, image_path, false)
        .await;
    let response = match result {
        Ok(response) => response,
        Err(err) => {
            if let Some(spin) = spinner {
                stop_spinner(spin);
            }
            return Err(err);
        }
    };
    if let Some(spin) = spinner {
        stop_spinner(spin);
    }

    // Always print the AI's response
    print_with_optional_formatting(
        persona.name.as_str(),
        model,
        &Utc::now(),
        response.as_str(),
        is_terminal,
    );

    // Print statistics only if output is to terminal
    if is_terminal {
        services.chat_service.print_statistics();
    }

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
        Ok(buffer.to_string())
    } else {
        get_multiline_input("Your message (end with Ctrl+D): ", vec![])
    }
}

fn get_config() -> Config {
    let config = config::CONFIG.lock().unwrap();
    config.clone()
}

fn get_last_session_name() -> Result<Option<String>, Box<dyn Error>> {
    let sessions = DirectoryChatStorage::new(get_chat_sessions_dir()?).list_sessions()?;
    Ok(sessions.last().cloned())
}
