use crate::chat::file_storage::NilChatStorage;
use crate::chat::service::ChatService;
use crate::config;
use crate::persona::Persona;
use std::process::Command;

pub async fn run_commitmessage() -> Result<(), Box<dyn std::error::Error>> {
    let model = {
        let config = config::CONFIG.lock().unwrap();
        config.ai.commit_model.clone()
    };
    let storage = NilChatStorage {};

    let mut chat_service = ChatService::builder()
        .model_name(&model)
        .storage(Box::new(storage))
        .persona(Persona {
            name: "git_expert".to_string(),
            chat_prompt: "Begin your message with a short summary of your changes (up to 50 characters as a guideline).
        Separate it from the following body by including a blank line.
        The body of your message should provide detailed answers to the following questions:
        – What was the motivation for the change?
        – How does it differ from the previous implementation?
        Use the imperative, present tense («change», not «changed» or «changes»)
        to be consistent with generated messages from commands like git merge.
    Following these rules write a concise but short commit message reflecting the following changes:
    ".to_string(),
            file_types: vec![],
            excluded_dirs: vec![],
        })
        .build()?;

    let diff = generate_git_diff_summary().await?;
    let summary = chat_service.send_message(&diff, false).await?;

    println!("Summary of git diff (model: {}):\n{}", &model, summary);

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
