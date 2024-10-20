use async_openai::config::OpenAIConfig;
use async_openai::types::{
    CreateImageRequestArgs, Image, ImageModel, ImageQuality, ImageResponseFormat, ImageSize,
};
use async_openai::Client;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use dotenvy::dotenv;
use std::error::Error;
use std::sync::Arc;

pub enum ImageFormat {
    Landscape,
    Portrait,
    Square,
}

pub async fn create_image(
    description: &str,
    format: ImageFormat,
) -> Result<Vec<u8>, Box<dyn Error>> {
    dotenv().ok(); // Load environment variables from .env file
    let openai_key = std::env::var("OPENAI_KEY")
        .expect("OPENAI_KEY must be set in .env file or environment variables");
    let client = Client::with_config(OpenAIConfig::default().with_api_key(openai_key));

    let image_size = match format {
        ImageFormat::Landscape => ImageSize::S1792x1024,
        ImageFormat::Portrait => ImageSize::S1024x1792,
        ImageFormat::Square => ImageSize::S1024x1024,
    };
    // Create image request
    let request = CreateImageRequestArgs::default()
        .prompt(description)
        .n(1) // Number of images to generate
        .size(image_size)
        .model(ImageModel::DallE3)
        .response_format(ImageResponseFormat::B64Json)
        .quality(ImageQuality::HD)
        .build()?;

    let response = client.images().create(request).await?;
    if response.data.is_empty() {
        return Err("No image was returned by the API".into());
    }

    // Decode the base64 image
    let b64_json = &*(response.data[0]);
    if let Image::B64Json { b64_json, .. } = &b64_json {
        let b64_json_str = Arc::clone(b64_json);
        Ok(BASE64_STANDARD.decode(&*b64_json_str)?)
    } else {
        Ok(Vec::new())
    }
}
