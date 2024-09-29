use crate::chat::file_storage::NilChatStorage;
use crate::chat::service::ChatService;
use crate::config;
use crate::openai_api::openai_interface::OpenAIInterface;
use crate::persona::get_personas;
use std::process::Command;

pub async fn run_commitmessage() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::CONFIG.lock().unwrap();
    let model = &config.ai.commit_model.clone();
    drop(config);
    let openai = OpenAIInterface::new(String::from(model));
    let storage = NilChatStorage {};

    let mut chat_service = ChatService::new(openai, storage, get_personas()[0].clone(), None);

    chat_service.add_system_message("
        Begin your message with a short summary of your changes (up to 50 characters as a guideline).
        Separate it from the following body by including a blank line.
        The body of your message should provide detailed answers to the following questions:
        – What was the motivation for the change?
        – How does it differ from the previous implementation?
        Use the imperative, present tense («change», not «changed» or «changes»)
        to be consistent with generated messages from commands like git merge.
    Following these rules write a concise but short commit message reflecting the following changes:
    ");

    let diff = generate_git_diff_summary().await?;
    let summary = chat_service.send_message(&diff, false).await?;

    println!("Summary of git diff (model: {}):\n{}", model, summary);

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
