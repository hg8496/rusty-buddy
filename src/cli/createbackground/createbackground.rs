use crate::cli::spinner::{start_spinner, stop_spinner};
use crate::cli::utils::get_multiline_input;
use async_openai::config::OpenAIConfig;
use async_openai::types::{CreateImageRequestArgs, Image, ImageModel, ImageQuality, ImageResponseFormat, ImageSize};
use async_openai::Client;
use dotenvy::dotenv;
use std::error::Error;
use std::fs;
use std::sync::Arc;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use crate::cli::createbackground::Orientation;

pub async fn run_create_background(
    output_file: &str,
    orientation: Orientation,
) -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // Load environment variables from .env file
    let openai_key = std::env::var("OPENAI_KEY")
        .expect("OPENAI_KEY must be set in .env file or environment variables");

    // Get user's description
    let prompt_message = "Please describe the background you wish to create. Type 'Ctrl+D' on a new line when you're finished:";
    let description = get_multiline_input(prompt_message)?;

    if description.trim().is_empty() {
        println!("No description provided. Exiting.");
        return Ok(());
    }

    let image_size = match orientation {
        Orientation::Landscape => ImageSize::S1792x1024,
        Orientation::Portrait => ImageSize::S1024x1792,
    };

    // Create OpenAI client
    let client = Client::with_config(OpenAIConfig::default().with_api_key(openai_key));

    // Create image request
    let request = CreateImageRequestArgs::default()
        .prompt(description.clone())
        .n(1) // Number of images to generate
        .size(image_size)
        .model(ImageModel::DallE3)
        .response_format(ImageResponseFormat::B64Json)
        .quality(ImageQuality::HD)
        .build()?;

    println!("Generating image...");

    // Start spinner
    let spinner = start_spinner();

    // Send request to OpenAI
    let response = client.images().create(request).await?;

    // Stop spinner
    stop_spinner(spinner);

    if response.data.is_empty() {
        println!("No image was returned by the API.");
        return Ok(());
    }

    // Decode the base64 image
    let b64_json = &*(response.data[0]);
    if let Image::B64Json { b64_json, .. } = &b64_json {
        let b64_json_str = Arc::clone(b64_json);
        let image_data = BASE64_STANDARD.decode(&*b64_json_str)?;

        let filepath = std::path::Path::new(output_file);
        fs::write(filepath, image_data)?;

        println!("Background image saved successfully.");
    }

    Ok(())
}
