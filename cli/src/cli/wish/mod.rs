//! This module provides the functionality for executing commands based on user-defined wishes within
//! the Rusty Buddy application. It allows users to express their wishes as natural language commands
//! and facilitates automatic file creation and manipulation as per the user's requests.
//! Utilizing the command-line arguments parsed into a `WishArgs` structure, it streamlines the interaction
//! between the user and the AI backend, making it easier to fulfill development tasks through simple commands.
//!
//! ## Key Components
//!
//! - **run:** An asynchronous function that takes `WishArgs` as input and delegates execution to
//!   the underlying `run_wish` function located in the `run` module. This central command processes
//!   the user's requests and interacts with the chat service to carry out the specified operations.
//!
//! ## Usage Example
//!
//! Here’s how to utilize the `run` function to execute a wish command:
//!
//! ```rust
//! use crate::cli::wish::{WishArgs, run};
//!
//! #[tokio::main]
//! async fn main() {
//!     let args = WishArgs {
//!         directory: "./src".to_string(),
//!         tools: true,
//!     };
//!     
//!     if let Err(e) = run(args).await {
//!         eprintln!("Error executing wish command: {}", e);
//!     }
//! }
//! ```
//!
//! This example demonstrates how to specify the directory and tool options, allowing Rusty Buddy to
//! fulfill the user’s request automatically, enhancing productivity and efficiency in the development process.
//!
//! ## Error Handling
//!
//! The `run` function returns a `Result` type, encapsulating either a successful execution (`Ok(())`)
//! or an error wrapped in a `Box` that indicates what went wrong during the request processing. Users
//! should be prepared to handle errors gracefully to ensure a smooth interaction experience.

mod run;
mod wish_args;

pub use wish_args::WishArgs;

pub async fn run(args: WishArgs) -> Result<(), Box<dyn std::error::Error>> {
    run::run_wish(args.directory, args.tools).await
}
