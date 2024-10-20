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
//! - `run_createicon`: Initiates the icon creation process based on user input.
//!
//! ## Examples
//!
//! Here's how to utilize the `run_createicon` function to create icons:
//!
//! ```rust
//! use crate::cli::createicon::{run_createicon};
//!
//! #[tokio::main]
//! async fn main() {
//!     let output_dir = "./icons";
//!     let sizes = vec![64, 128, 256];
//!     let description = "Design a modern icon"; // You would typically receive this from the user
//!     
//!     if let Err(e) = run_createicon(output_dir, sizes).await {
//!         eprintln!("Error generating icons: {}", e);
//!     }
//! }
//! ```
//!
//! This example demonstrates how to specify the output directory and sizes through the
//! `run_createicon` function, invoking it to generate icons.
//!
//! ## Important Notes
//!
//! - The function interacts with the OpenAI API to create an icon image based on a user-provided description.
//! - It checks whether a description is provided either through standard input or prompts the user for one.
//! - After obtaining the image, it saves the original and resized versions to the designated output directory.
//! - The module emphasizes user input and flexibility, allowing users to tailor the icon
//!   creation process according to their project needs.
use crate::cli::editor::get_multiline_input;
use crate::cli::spinner::{start_spinner, stop_spinner};
use atty::Stream;
use rbchat::image::{create_image, ImageFormat};
use std::error::Error;
use std::io::Read;
use std::path::Path;
use std::{fs, io};

/// This function generates an icon image based on a user-provided description
/// using the OpenAI API. It takes an output directory to save the generated
/// icon images in various specified sizes. If the description is provided via
/// standard input, it will read from there; otherwise, it prompts the user for
/// input. The function checks if the description is non-empty, initializes an
/// OpenAI client, and sends a request to create the image. After receiving the
/// response, it decodes the image from base64, saves the original image, and
/// resizes it to the specified dimensions. It prints status updates throughout
/// the process.
///
/// # Parameters
/// - `output_dir`: A string slice that represents the directory where the icons
///   will be saved.
/// - `sizes`: A vector of unsigned integers that represent the sizes of the icons
///   to be generated.
///
/// # Returns
/// Returns a Result that is Ok(()) on success or an error if any step fails.
pub async fn run_createicon(output_dir: &str, sizes: Vec<u32>) -> Result<(), Box<dyn Error>> {
    let description = if !atty::is(Stream::Stdin) {
        // Read from standard input if it's not a terminal (piped data)
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer.trim().to_string() // Use the piped content as the description
    } else {
        // Get user's description
        let prompt_message = "Please describe the icon you wish to create. Type 'Ctrl+D' on a new line when you're finished:";
        get_multiline_input(prompt_message, vec![])?
    };

    if description.trim().is_empty() {
        println!("No description provided. Exiting.");
        return Ok(());
    }

    println!("Generating image...");

    // Start spinner
    let spinner = start_spinner();

    // Stop spinner
    stop_spinner(spinner);
    let image_data = create_image(description.as_str(), ImageFormat::Square).await?;

    // Load the image
    let img = image::load_from_memory(&image_data)?;

    // Ensure output directory exists
    fs::create_dir_all(output_dir)?;

    // Save the original image
    let original_path = Path::new(output_dir).join("icon_original.png");
    img.save(&original_path)?;

    println!("Original image saved to {:?}", original_path.display());

    // Generate and save icons in specified sizes
    for size in sizes {
        let resized_img = img.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
        let filename = format!("icon_{}x{}.png", size, size);
        let filepath = Path::new(output_dir).join(filename);
        resized_img.save(&filepath)?;

        println!(
            "Icon of size {}x{} saved to {:?}",
            size,
            size,
            filepath.display()
        );
    }
    println!("Icon creation complete.");

    Ok(())
}
