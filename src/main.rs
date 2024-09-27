use std::io;
use clap::{value_parser, Arg, ArgAction, Command, ValueHint};
use clap_complete::aot::{generate, Generator, Shell};
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

    let cmd = build_cli();
    let matches = cmd.get_matches();

    if let Some(generator) = matches.get_one::<Shell>("completion").copied() {
        print_completions(generator, &mut build_cli());
    } else if let Some(createicon_matches) = matches.subcommand_matches("createicon") {
        let output_dir = createicon_matches.get_one::<String>("output").unwrap();
        let sizes = createicon_matches
            .get_one::<String>("sizes")
            .unwrap()
            .split(',')
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect::<Vec<_>>();
        cli::run_createicon(output_dir, sizes).await.unwrap();
    } else if let Some(_) = matches.subcommand_matches("commitmessage") {
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

fn build_cli() -> Command {
    Command::new("rusty-buddy")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Christian Stolz <hg8496@cstolz.de>")
        .about("A command line interface for various tasks")
        .arg(
            Arg::new("completion")
                .long("completion")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
        )
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
                        .help("Directory to add to the chat context")
                        .value_hint(ValueHint::AnyPath),
                ),
        )
        .subcommand(
            Command::new("createicon")
                .about("Create an icon using DALL·E based on user input")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("DIRECTORY")
                        .default_value("./icons")
                        .help("Output directory for the generated icons"),
                )
                .arg(
                    Arg::new("sizes")
                        .short('s')
                        .long("sizes")
                        .value_name("SIZES")
                        .help("Comma-separated list of icon sizes to generate")
                        .default_value("16,32,64,128,256,512")
                        .action(ArgAction::Set),
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
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
