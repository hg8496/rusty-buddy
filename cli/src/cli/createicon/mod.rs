//! This module provides functionality to run the create icon process asynchronously.
//!
//! It takes `CreateIconArgs`, which includes parameters such as output paths and sizes
//! for the generated icons. The module leverages the underlying `run` functionality to
//! execute the creation process, facilitating seamless integration of icon generation
//! within the Rusty Buddy toolset.
//!
//! ## Overview
//!
//! The `createicon` module allows users to specify various parameters for icon creation
//! including output directory and icon sizes. It provides a user-friendly interface to
//! customize and generate icons based on specified criteria, integrating with AI capabilities
//! to enhance productivity.
//!
//! ### Key Functions
//!
//! - `run`: Initiates the icon creation process.
//!
//! ## Examples
//!
//! Here's how to utilize the `run` function to create icons:
//!
//! ```rust
//! use crate::cli::createicon::{CreateIconArgs, run};
//!
//! #[tokio::main]
//! async fn main() {
//!     let args = CreateIconArgs {
//!         output: "./icons".to_string(),
//!         sizes: vec![64, 128, 256],
//!     };
//!     
//!     if let Err(e) = run(args).await {
//!         eprintln!("Error generating icons: {}", e);
//!     }
//! }
//! ```
//!
//! This example demonstrates how to specify the output directory and sizes through the
//! `CreateIconArgs` struct, invoking the `run` function to generate icons.
//!
//! ## Additional Notes
//!
//! The module emphasizes user input and flexibility, allowing users to tailor the icon
//! creation process according to their project needs. This aligns with Rusty Buddy's goal
//! of enhancing development workflows through powerful AI-driven functionalities.
mod ci_args;
mod run;

pub use ci_args::CreateIconArgs;

pub async fn run(args: CreateIconArgs) -> Result<(), Box<dyn std::error::Error>> {
    crate::cli::createicon::run::run_createicon(args.output.as_str(), args.sizes).await
}
