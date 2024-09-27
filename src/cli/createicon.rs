use crate::cli::spinner::{start_spinner, stop_spinner};
use crate::cli::utils::get_user_input;
use async_openai::config::OpenAIConfig;
use async_openai::types::{CreateImageRequestArgs, Image, ImageResponseFormat, ImageSize};
use async_openai::Client;
use base64::prelude::*;
use dotenvy::dotenv;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::sync::Arc;

pub async fn run_createicon(output_dir: &str, sizes: Vec<u32>) -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // Load environment variables from .env file
    let openai_key = std::env::var("OPENAI_KEY")
        .expect("OPENAI_KEY must be set in .env file or environment variables");

    // Get user's description
    let prompt_message = "Please describe the icon you wish to create. Type 'Ctrl+D' on a new line when you're finished:";
    let description = get_user_input(prompt_message)?;

    if description.trim().is_empty() {
        println!("No description provided. Exiting.");
        return Ok(());
    }

    // Create OpenAI client
    let client = Client::with_config(OpenAIConfig::default().with_api_key(openai_key));

    // Create image request
    let request = CreateImageRequestArgs::default()
        .prompt(description.clone())
        .n(1) // Number of images to generate
        .size(ImageSize::S1024x1024) // Largest size, we'll downscale as needed
        .response_format(ImageResponseFormat::B64Json)
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
        // Dereference or clone the Arc to get a String
        let b64_json_str = Arc::clone(b64_json);
        let image_data = BASE64_STANDARD.decode(&*b64_json_str)?;

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
    }
    println!("Icon creation complete.");

    Ok(())
}
