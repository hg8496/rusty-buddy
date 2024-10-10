//! Rusty Buddy is an AI-powered command-line interface (CLI) tool designed to enhance development workflows by providing various operations related to personas, chat functionality, and other commands.
//!
//! This application utilizes the `clap` library for robust command-line argument parsing and the `tokio` library for asynchronous execution. The `main` function serves as the entry point, initializing the logger, processing command-line arguments, and executing the corresponding command based on user input.
//!
//! ## Key Features
//!
//! - **Initialization:** Set up configuration and environment for Rusty Buddy.
//! - **Command Execution:** Supports commands for generating commit messages, starting chat sessions, creating icons and backgrounds, and expressing wishes for automated file manipulations.
//! - **Persona Management:** Offers a mechanism to list existing personas and allows users to select a specific persona for tailored interactions.
//! - **Command Completion:** Generates shell completion scripts to enhance user experience by providing command suggestions.
//!
//! ## Usage Example
//!
//! ```bash
//! # Start a new chat session
//! rusty-buddy chat --new
//!
//! # Generate a commit message from staged changes
//! git add .
//! rusty-buddy commit-message
//! ```
//!
//! ## Modules
//!
//! - `args`: Defines the argument structure for command-line interface management.
//! - `chat`: Handles chat interactions, including session management and persona integration.
//! - `cli`: Responsible for command execution and general command-line interface functions.
//! - `config`: Manages configuration settings and file operations.
//! - `persona`: Defines and provides functionality related to user personas.
//! - `provider`: Interfaces with different AI backends like OpenAI and Ollama.

mod args;
mod chat;
mod cli;
mod config;
mod context;
mod persona;
mod provider;

use crate::cli::init::run_init_command;
use crate::config::get_config_file;
use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator};
use log::info;
use std::io;

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    // Example info log
    info!("Application started");
    let cli = args::Cli::parse();
    if let Some(args::Commands::Init) = cli.command {
        run_init_command().await.unwrap();
    }
    if !check_environment() {
        eprintln!("No configuration file found.");
        std::process::exit(1);
    }
    if cli.list_personas {
        persona::print_all_personas();
        return;
    }
    if let Some(completion) = cli.completion {
        print_completions(completion, &mut args::Cli::command());
    } else if let Some(command) = cli.command {
        match command {
            args::Commands::CommitMessage(args) => {
                cli::commitmessage::run(args).await.unwrap();
            }
            args::Commands::Chat(args) => {
                cli::chat::run(args).await.unwrap();
            }
            args::Commands::CreateIcon(args) => {
                cli::createicon::run(args).await.unwrap();
            }
            args::Commands::CreateBackground(args) => {
                cli::createbackground::run(args).await.unwrap();
            }
            args::Commands::Wish(args) => {
                cli::wish::run(args).await.unwrap();
            }
            args::Commands::Init => {}
        }
    } else {
        println!("No valid command given. Use `rusty-buddy help` for more information.");
    }
}

fn check_environment() -> bool {
    get_config_file().is_ok()
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
