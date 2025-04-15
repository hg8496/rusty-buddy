//! This module defines the basic argument structure for Rusty Buddy, an AI-powered command-line tool designed to enhance development workflows.
//!
//! It utilizes the `clap` library to provide a user-friendly interface for parsing command-line arguments and options. The main entry point is the `Cli` struct, which organizes the available commands and their respective arguments.
//!
//! Through the `Cli` struct, users can activate shell completions, list available personas, and execute various commands meant to interface with specialized functionalities of Rusty Buddy. Each command may accept specific arguments tailored to its intended operation, making the tool flexible and adaptable to various developer needs.
//!
//! # Example Usage
//!
//! To execute the program and parse arguments, the user can run:
//!
//! ```bash
//! rusty-buddy --list-personas
//! ```
//!
//! This command will list all available personas that can be engaged within the chat functionality, illustrating how users can interact with the tool.
//!
//! # Components
//!
//! - `Cli`: The main structure representing the command-line interface.
//! - `Commands`: An enumeration of different subcommands that the user can invoke.
//! - `ChatArgs`, `CommitMessageArgs`, `BackgroundArgs`, `CreateIconArgs`, and `WishArgs`: Additional argument structures for each command.
//!
//! Each subcommand can also be detailed and equipped with functionality specific to the command's purpose.

use crate::cli::chat::ChatArgs;
use crate::cli::commitmessage::CommitMessageArgs;
use crate::cli::createbackground::BackgroundArgs;
use crate::cli::createicon::CreateIconArgs;
use crate::cli::init::InitArgs;
use crate::cli::knowledge::KnowledgeArgs;
use crate::cli::wish::WishArgs;
use clap::{Parser, Subcommand};
use clap_complete::aot::Shell;

/// This struct represents the command-line interface (CLI) for Rusty Buddy.
#[derive(Parser)]
#[command(
    name = "rusty-buddy",
    author = "Christian Stolz <hg8496@cstolz.de>",
    about = "A command line interface to empower your development workflow with AI",
    version
)]
pub struct Cli {
    /// Activate shell completion generation
    #[arg(long, value_enum)]
    pub completion: Option<Shell>,

    /// List all available personas
    #[arg(long)]
    pub list_personas: bool,

    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available commands that can be executed through the CLI.
#[derive(Subcommand)]
pub enum Commands {
    /// Summarize the output of `git diff`.
    CommitMessage(CommitMessageArgs),

    /// Start, continue, or load a chat session.
    Chat(ChatArgs),

    /// Create an icon using DALL·E based on user input.
    CreateIcon(CreateIconArgs),

    /// Create a background using DALL·E based on user input.
    CreateBackground(BackgroundArgs),

    /// Collect files from a specified directory and create a context for chat.
    Wish(WishArgs),

    /// Manage knowledge entries.
    #[clap(subcommand)]
    Knowledge(KnowledgeArgs),

    /// Initialize configuration and environment.
    Init(InitArgs),
}
