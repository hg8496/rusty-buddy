mod args;
mod chat;
mod cli;
mod config;
mod openai_api;
mod persona;

use crate::cli::init::run_init_command;
use crate::config::get_config_file;
use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator};
use dotenvy::dotenv;
use std::io;

#[tokio::main]
async fn main() {
    let cli = args::Cli::parse();
    if let Some(args::Commands::Init) = cli.command {
        run_init_command().await.unwrap();
    }
    dotenv()
        .map_err(|e| eprintln!("Failed to load .env file: {}", e))
        .unwrap();
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
            args::Commands::Init => {
                run_init_command().await.unwrap();
            }
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
