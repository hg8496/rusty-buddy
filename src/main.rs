mod args;
mod chat;
mod cli;
mod config;
mod openai_api;
mod persona;

use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator};
use dotenvy::dotenv;
use std::io;

#[tokio::main]
async fn main() {
    dotenv()
        .map_err(|e| eprintln!("Failed to load .env file: {}", e))
        .unwrap();

    let cli = args::Cli::parse(); // Parse command line arguments with the new CLI struct

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
        }
    } else {
        println!("No valid command given. Use `rusty-buddy help` for more information.");
    }
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
