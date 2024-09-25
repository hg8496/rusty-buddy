use clap::{Arg, ArgAction, Command};
use dotenvy::dotenv;

mod chat;
mod cli; // Import the CLI module
mod config;
mod openai_api;
mod persona;

#[tokio::main]
async fn main() {
    dotenv()
        .map_err(|e| eprintln!("Failed to load .env file: {}", e))
        .unwrap();

    let matches = Command::new("Rusty CLI")
        .version("1.0")
        .author("Christian Stolz <hg8496@cstolz.de>")
        .about("A command line interface for various tasks")
        .subcommand(Command::new("commitmessage").about("Summarize the output of `git diff`."))
        .subcommand(
            Command::new("chat")
                .about("Start, continue, or load a chat session.")
                .arg(
                    Arg::new("new")
                        .short('n')
                        .long("new")
                        .action(ArgAction::SetTrue)
                        .help("Start a new chat session"),
                )
                .arg(
                    Arg::new("continue")
                        .short('c')
                        .long("continue")
                        .action(ArgAction::SetTrue)
                        .help("Continue the last chat session"),
                )
                .arg(
                    Arg::new("load")
                        .short('l')
                        .long("load")
                        .help("Load a specific chat session by name"),
                )
                .arg(
                    Arg::new("directory")
                        .short('d')
                        .long("directory")
                        .required(false)
                        .help("Directory to add to the chat context"),
                ),
        )
        .subcommand(
            Command::new("wish")
                .arg(
                    Arg::new("directory")
                        .required(true)
                        .help("The directory to collect files from"),
                )
                .arg(
                    Arg::new("tools")
                        .short('t')
                        .long("tools")
                        .action(ArgAction::SetTrue)
                        .help("Activate the usage of tools"),
                )
                .about("Collect files from a specified directory and create a context for chat."),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("commitmessage") {
        cli::run_commitmessage().await.unwrap();
    } else if let Some(chat_matches) = matches.subcommand_matches("chat") {
        let start_new = chat_matches.get_flag("new");
        let continue_last = chat_matches.get_flag("continue");
        let load_name = chat_matches.get_one::<String>("load").cloned();
        let directory = chat_matches.get_one::<String>("directory").cloned();

        cli::run_chat(start_new, continue_last, load_name, directory)
            .await
            .unwrap();
    } else if let Some(wish_matches) = matches.subcommand_matches("wish") {
        if let Some(directory) = wish_matches.get_one::<String>("directory") {
            let use_tools = wish_matches.get_flag("tools");
            cli::run_wish(directory, use_tools).await.unwrap();
        }
    } else {
        println!("No valid command given. Use `rusty-buddy help` for more information.");
    }
}
