use crate::chat::file_storage::NilChatStorage;
use crate::chat::service::ChatService;
use crate::openai_api::openai_interface::OpenAIInterface;
use std::process::Command;

pub async fn run_commitmessage() -> Result<(), Box<dyn std::error::Error>> {
    let openai = OpenAIInterface::new();
    let storage = NilChatStorage {};
    let mut chat_service = ChatService::new(openai, storage);

    // Add a system message to the chat context
    chat_service.add_system_message("Please write a commit message for the following git diff, place the summary at the beginning:");

    let diff = generate_git_diff_summary().await?;

    // Send the diff to the ChatService
    let summary = chat_service.send_message(&diff, false).await?;

    println!("Summary of git diff:\n{}", summary);

    Ok(())
}

pub async fn generate_git_diff_summary() -> Result<String, Box<dyn std::error::Error>> {
    // Capture the output of `git diff`
    let output = Command::new("git").arg("diff").arg("--cached").output()?;

    if !output.status.success() {
        return Err("Failed to execute git diff".into());
    }

    let diff_output = String::from_utf8_lossy(&output.stdout);

    Ok(diff_output.to_string())
}
