//! This module defines the `ChatArgs` structure, which encapsulates the command-line arguments
//! for managing a chat session in the Rusty Buddy application. Leveraging the `clap` library,
//! it facilitates sophisticated parsing and control of various options that can influence
//! the behavior and context of the chat application.
//!
//! ## Overview
//!
//! The `ChatArgs` struct comprises several parameters that govern how a chat session
//! should be launched and configured within the application. Options include starting new sessions,
//! resuming previous ones, sending immediate messages, and more. Each field is meticulously documented
//! to clarify expected user inputs via command-line arguments.
//!
//! ## Components
//!
//! - `new`: Signals the start of a new chat session.
//! - `continue_last`: Enables the resumption of the most recent active chat session.
//! - `load`: Designates a named session to load.
//! - `directory`: Specifies a directory to integrate into the chat context.
//! - `persona`: Denotes a particular persona for interacting with the AI.
//! - `one_shot`: Sends a single message, exiting the session immediately.
//! - `model`: Defines the AI model to be used during the chat session.
//! - `silence`: Suppresses outputs of prior messages when loading sessions.
//!
//! ## Usage Example
//!
//! Below is a simple example demonstrating how to initialize a chat session using `ChatArgs`:
//!
//! ```rust
//! use crate::cli::chat::ChatArgs;
//!
//! let args = ChatArgs {
//!     new: true,
//!     continue_last: false,
//!     load: None,
//!     directory: Some(String::from("./src")),
//!     persona: Some(String::from("rusty")),
//!     one_shot: None,
//!     model: Some(String::from("gpt-3.5")),
//!     silence: false,
//! };
//!
//! // Further code to utilize args in initiating the session...
//! ```
//!
//! This structure is integral in setting up chat-related functionalities,
//! thereby aiding developers in enhancing the interactive elements of Rusty Buddy.

use clap::Args;
use std::path::PathBuf;

/// Structure representing command-line arguments for managing a chat session.
///
/// This struct employs the Clap library for parsing command-line arguments and offers numerous
/// options to dictate the behavior of the chat application. The arguments empower the user
/// to initiate a new chat session, resume an existing one, load a specific session, designate
/// a directory for context, select a persona, send a one-time message, specify an AI model,
/// or suppress old message outputs.
///
/// The accessible arguments are:
/// - `new`: Initiate a new chat session.
/// - `continue_last`: Resume the last chat session.
/// - `load`: Load a specific chat session by name.
/// - `directory`: Indicate a directory to add to the chat context.
/// - `persona`: Choose a persona for the chat session.
/// - `one_shot`: Dispatch one message and exit.
/// - `model`: Define the AI model for the chat session.
/// - `silence`: Suppress the output of old messages.
#[derive(Args)]
pub struct ChatArgs {
    /// Start a new chat session
    #[arg(short, long)]
    pub new: bool,

    /// Continue the last chat session
    #[arg(short, long, action)]
    pub continue_last: bool,

    /// Load a specific chat session by name
    #[arg(short, long)]
    pub load: Option<String>,

    /// Directories to add to the chat context
    ///
    /// Can be specified multiple times.
    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    pub directory: Option<Vec<PathBuf>>,

    /// Specify a persona for the chat session
    #[arg(short, long)]
    pub persona: Option<String>,

    /// Send one message and quit. If \[\<ONE_SHOT\>\] is specified, use it as user input.
    #[arg(short = 'o', long = "one-shot")]
    pub one_shot: Option<Option<String>>,

    /// Sets the AI model to use in this chat session
    #[arg(short = 'm', long = "model")]
    pub model: Option<String>,

    /// Silence the output of old messages
    #[arg(short, long)]
    pub silence: bool,

    /// Add relevant knowledge from the knowledge database to the session.
    ///
    /// This option uses the latest user input to generate embeddings and search
    /// the knowledge store for relevant documents, which will be included as
    /// knowledge context messages before the assistant responds.
    #[arg(short = 'k', long = "knowledge")]
    pub knowledge: Option<Option<usize>>,

    /// Image file to add to the chat. Only works with vision capable models!                            
    #[arg(short = 'i', long = "image", value_hint = clap::ValueHint::FilePath)]
    pub image: Option<PathBuf>,
}
