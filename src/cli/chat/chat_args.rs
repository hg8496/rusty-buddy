//! This module defines the `ChatArgs` structure which holds the command-line arguments
//! related to managing a chat session in the Rusty Buddy application. Utilizing the
//! `clap` library, it allows for flexible parsing and control of various options that
//! affect the behavior and context of the chat application.
//!
//! ## Overview
//!
//! The `ChatArgs` struct encompasses several options that dictate how the chat session
//! should be initialized and its parameters configured within the application. This
//! includes the ability to start new sessions, continue previous ones, or send one-off
//! messages directly. Each field is annotated to provide clarity regarding expected
//! user input through command-line arguments.
//!
//! ## Components
//!
//! - `new`: Indicates whether to start a new chat session.
//! - `continue_last`: Allows the continuation of the last active chat session.
//! - `load`: Specifies the name of a saved session to load.
//! - `directory`: Defines a directory to be added to the context of the chat.
//! - `persona`: Specifies a particular persona under which to interact with the AI.
//! - `one_shot`: Allows sending a single message and exiting the session immediately.
//! - `message`: Provides a specific message to be used as input for one-off queries.
//! - `silence`: Silences any output of previous messages when loading sessions.
//!
//! ## Usage Example
//!
//! Here is a basic example of how you might initialize a chat session using the `ChatArgs`:
//!
//! ```rust
//! use crate::cli::chat::ChatArgs;
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
//! // Further code to utilize args in starting the session...
//! ```
//!
//! This structure provides the foundational setup for chat-related functionality,
//! making it easier for developers to enhance the interactive capabilities of Rusty Buddy.
use clap::Args;

/// Structure representing command-line arguments for managing a chat session.
///
/// This struct uses the Clap library to parse command-line arguments and provides various
/// options to control the behavior of the chat application. The arguments allow the user
/// to start a new chat session, continue an existing one, load a specific session, set
/// a directory for context, specify a persona, send a one-time message, or silence old
/// message outputs.
///
/// The available arguments are:
/// - `new`: Start a new chat session.
/// - `continue_last`: Continue the last chat session.
/// - `load`: Load a specific chat session by name.
/// - `directory`: Specify a directory to add to the chat context.
/// - `persona`: Specify a persona for the chat session.
/// - `one_shot`: Send one message and exit.
/// - `message`: Use a specific message as user input.
/// - `silence`: Silence the output of old messages.
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

    /// Directory to add to the chat context
    #[arg(short, long)]
    pub directory: Option<String>,

    /// Specify a persona for the chat session
    #[arg(short, long)]
    pub persona: Option<String>,

    /// Send one message and quit
    #[arg(short = 'o', long = "one-shot")]
    pub one_shot: bool,

    /// Use this message as user input
    #[arg(short = 'm', long = "message")]
    pub message: Option<String>,

    /// Silence the output of old messages
    #[arg(short = 's', long = "silence")]
    pub silence: bool,
}
