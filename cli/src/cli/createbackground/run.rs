//! This module provides functionality for generating background images based on user descriptions
//! or input from standard input. It utilizes the OpenAI API to create images in specified
//! orientations (landscape or portrait) and saves them to the given output file. The process
//! includes reading user input, making API requests, and handling image data.
//!
//! # Overview
//!
//! The core function of this module is `run_create_background`, which is executed asynchronously.
//! This function handles the following tasks:
//! - Reads a description for the background image from the user or standard input.
//! - Generates a request to the OpenAI API to create the image.
//! - Handles image data, including base64 decoding and file writing.
//!
//! ## Key Functions
//!
//! - `run_create_background`: Generates a background image based on the user's description.
//!
//! # Usage
//!
//! The module is designed to be used where background image generation is needed. It expects
//! the OpenAI API key to be provided through environment variables.
//!
//! ## Example
//!
//! ```rust
//! use crate::cli::createbackground::{Orientation, run_create_background};
//!
//! #[tokio::main]
//! async fn main() {
//!     let output_file = "path/to/output/background.png";
//!     let orientation = Orientation::Landscape;
//!     if let Err(e) = run_create_background(output_file, orientation).await {
//!         eprintln!("Error generating background: {}", e);
//!     }
//! }
//! ```
//!
//! ## Error Handling
//!
//! The `run_create_background` function will return an error if:
//! - The OpenAI API key is not set in the environment variables.
//! - The image request to the API fails.
//! - There are issues with file writing the resulting image.
//!
//! This module leverages the dotenvy crate to load environment variables, ensuring the API key
//! is securely handled.

use crate::cli::createbackground::Orientation;
use crate::cli::editor::get_multiline_input;
use crate::cli::spinner::{start_spinner, stop_spinner};
use atty::Stream;
use rbchat::image::{create_image, ImageFormat};
use std::error::Error;
use std::io::Read;
use std::{fs, io};

/// Asynchronously generates a background image based on a user-provided description or input from stdin,
/// and saves it to the specified output file. The orientation can be set to either Landscape or Portrait,
/// and it utilizes OpenAI's image generation capabilities. If the description is empty, or no image is returned,
/// appropriate messages will be displayed to the user.
///
/// # Arguments
///
/// * `output_file` - A string slice that represents the file path to save the generated background image.
/// * `orientation` - The orientation of the image, which can be Landscape or Portrait.
///
/// # Returns
///
/// This function returns a Result that, on success, contains an empty tuple,
/// or an error wrapped in a Box if something goes wrong.
pub async fn run_create_background(
    output_file: &str,
    orientation: Orientation,
) -> Result<(), Box<dyn Error>> {
    let description = if !atty::is(Stream::Stdin) {
        // Read from standard input if it's not a terminal (piped data)
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer.trim().to_string() // Use the piped content as the description
    } else {
        // Get user's description
        let prompt_message = "Please describe the background you wish to create. Type 'Ctrl+D' on a new line when you're finished:";
        get_multiline_input(prompt_message, vec![])?
    };

    if description.trim().is_empty() {
        println!("No description provided. Exiting.");
        return Ok(());
    }

    let image_size = match orientation {
        Orientation::Landscape => ImageFormat::Landscape,
        Orientation::Portrait => ImageFormat::Portrait,
    };

    println!("Generating image...");

    // Start spinner
    let spinner = start_spinner();
    let image_data = create_image(description.as_str(), image_size).await?;
    let filepath = std::path::Path::new(output_file);
    fs::write(filepath, image_data)?;

    println!("Background image saved successfully.");
    // Stop spinner
    stop_spinner(spinner);

    Ok(())
}
