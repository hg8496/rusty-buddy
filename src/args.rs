use crate::cli::chat::ChatArgs;
use crate::cli::commitmessage::CommitMessageArgs;
use crate::cli::createicon::CreateIconArgs;
use crate::cli::createbackground::BackgroundArgs;
use crate::cli::wish::WishArgs;
use clap::{Parser, Subcommand};
use clap_complete::aot::Shell;

#[derive(Parser)]
#[command(
    name = "rusty-buddy",
    version = "0.2.0",
    author = "Christian Stolz <hg8496@cstolz.de>",
    about = "A command line interface for various tasks",
    version
)]
pub struct Cli {
    /// Activate shell completion generation
    #[arg(long, value_enum)]
    pub completion: Option<Shell>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Summarize the output of `git diff`.
    CommitMessage(CommitMessageArgs),

    /// Start, continue, or load a chat session.
    Chat(ChatArgs),

    /// Create an icon using DALL·E based on user input.
    CreateIcon(CreateIconArgs),

    /// Create a background using DALL·E based on user input.
    CreateBackground(BackgroundArgs), // <-- New command

    /// Collect files from a specified directory and create a context for chat.
    Wish(WishArgs),
}
