use crate::cli::chat::commands::copy_files::CopyFilesCommand;
use crate::cli::chat::commands::copy_last_answer::CopyLastMessageCommand;
use crate::cli::chat::commands::save_files::SaveFilesCommand;
use crate::cli::chat::commands::save_last_answer::SaveLastAnswerCommand;
use rbchat::chat::command_registry::CommandRegistry;
use rbchat::chat::RegisterableCommand;

pub mod copy_files;
pub mod copy_last_answer;
mod message_files;
pub mod save_files;
pub mod save_last_answer;

pub fn initialize_cli_commands(registry: &mut CommandRegistry) {
    // Each command registers itself
    SaveFilesCommand::register_with_registry(registry);
    SaveLastAnswerCommand::register_with_registry(registry);
    CopyFilesCommand::register_with_registry(registry);
    CopyLastMessageCommand::register_with_registry(registry);
}
