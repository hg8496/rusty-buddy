pub mod chat;
pub mod commitmessage;
mod createicon;
mod spinner;
mod style;
pub mod utils;
pub mod wish;

pub async fn run_chat(
    start_new: bool,
    continue_last: bool,
    load_name: Option<String>,
    directory: Option<String>,
    persona_name: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    chat::run_chat(start_new, continue_last, load_name, directory, persona_name).await
}

pub async fn run_commitmessage() -> Result<(), Box<dyn std::error::Error>> {
    commitmessage::run_commitmessage().await
}

pub async fn run_wish(directory: &str, use_tools: bool) -> Result<(), Box<dyn std::error::Error>> {
    wish::run_wish(directory, use_tools).await
}

pub async fn run_createicon(
    output_dir: &str,
    sizes: Vec<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    createicon::run_createicon(output_dir, sizes).await
}
