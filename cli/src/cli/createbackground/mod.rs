//! This module provides functionality to run background tasks based on specified arguments.
//!
//! It imports arguments from the `bg_args` module and uses the `run` module to create a background task.
//!
//! The `run` function takes `BackgroundArgs`, which includes parameters such as the file to process
//! and its orientation and then calls the `run_create_background` function to handle the execution.
//!
//! ## Overview
//!
//! The primary purpose of this module is to facilitate the creation of background images using the OpenAI API.
//! Users can specify various options, such as the output file and its orientation (landscape or portrait).
//! This module ensures that the image generation process is seamless and robust, providing all necessary
//! configurations to produce visually appealing backgrounds based on user input.
//!
//! ### Functions
//!
//! - `run`: This function takes `BackgroundArgs` as input and generates a background image
//!   according to the provided specifications.
//!
//! ## Example Usage
//!
//! Here's how you might call the `run` function from this module:
//!
//! ```rust
//! use crate::cli::createbackground::{BackgroundArgs, run};
//!
//! #[tokio::main]
//! async fn main() {
//!     let args = BackgroundArgs {
//!         file: "./background.png".to_string(),
//!         orientation: Orientation::Landscape,
//!     };
//!
//!     run(args).await.unwrap();
//! }
//! ```
//!
//! In this example, we're generating a background image stored at `./background.png` in landscape orientation.
//!
//! ## Remarks
//!
//! It is essential to ensure that all parameters are provided correctly to avoid errors during image creation.
//! The module handles the OpenAI API interactions, and users should have their environment set up
//! properly with valid API keys stored in a `.env` file.
mod bg_args;
mod run;

pub use bg_args::{BackgroundArgs, Orientation};

pub async fn run(args: BackgroundArgs) -> Result<(), Box<dyn std::error::Error>> {
    run::run_create_background(args.file.as_str(), args.orientation).await
}
