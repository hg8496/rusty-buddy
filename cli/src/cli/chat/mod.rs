//! This module serves as the entry point for the chat application functionality within Rusty Buddy.
//!
//! It provides the `ChatArgs` structure, which is used for parsing and holding command-line arguments
//! specific to the chat interface. Through this module, users can interact with Rusty Buddy's AI
//! personality and context-aware capabilities, facilitating dynamic conversations by
//! leveraging specialized personas to enhance the user experience.
//!
//! ## Key Components
//!
//! - **ChatArgs**: A structure representing command-line arguments for starting and managing chat sessions.
//! - **run**: An asynchronous function that takes `ChatArgs` as input and initiates the chat process.
//!
//! ## Overview
//!
//! The `run` function orchestrates the chat process, managing context setup, message handling,
//! and interaction with the specified AI backend. It allows users to engage in rich, informative
//! conversations and supports various features, such as continuing previous sessions, loading defined
//! contexts, and utilizing personas for tailored interaction styles.
//!
//! ## Usage Example
//!
//! Here’s how you might initialize and run a new chat session:
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
//! In this example, a new chat session is started using the `rust` persona, allowing direct
//! interaction with the AI based on the user’s specifications.

mod chat_args;
mod commands;
mod run;

pub use chat_args::ChatArgs;

pub async fn run(args: ChatArgs) -> Result<(), Box<dyn std::error::Error>> {
    run::run_chat(args).await
}
