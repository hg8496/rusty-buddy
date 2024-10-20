//! This module provides functionality for generating commit messages using the specified arguments.
//!
//! It includes the `run` function, which serves as the primary entry point for executing the commit
//! message command. The command utilizes the `CommitMessageArgs` structure to define and parse the
//! relevant command-line arguments. The execution of the command is handled by the `run` module,
//! which performs the necessary operations to generate the commit message based on the current
//! state of the git repository.
//!
//! ## Usage
//!
//! Here's how you can execute the commit message command:
//!
//! ```rust
//! use crate::cli::commitmessage::{CommitMessageArgs, run};
//!
//! #[tokio::main]
//! async fn main() {
//!     let args = CommitMessageArgs { /* populate with command-line arguments */ };
//!     
//!     if let Err(e) = run(args).await {
//!         eprintln!("Error generating commit message: {}", e);
//!     }
//! }
//! ```
//!
//! In this example, we initialize `CommitMessageArgs` and run the command asynchronously. It captures
//! any errors that may occur during the process, providing a clear message if something goes wrong.
//!
//! ## Important Notes
//!
//! - The command retrieves information from the git staging area to generate contextually appropriate
//!   commit messages.
//! - It is essential to ensure that changes are staged using `git add` before executing this command.
//! - The commit message follows best practices and is designed to enhance the clarity and quality of
//!   commit history in a version control system.

mod cm_args;
mod run;

pub use cm_args::CommitMessageArgs;

pub async fn run(_args: CommitMessageArgs) -> Result<(), Box<dyn std::error::Error>> {
    run::run_commitmessage().await
}
