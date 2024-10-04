use crate::chat::command::RegisterableCommand;
use crate::chat::command_registry::CommandRegistry;
use crate::chat::commands::refresh::RenewCommand;
use crate::chat::commands::save_files::SaveFilesCommand;
use crate::chat::commands::save_last_answer::SaveLastAnswerCommand;

mod refresh;
mod save_files;
mod save_last_answer;

pub fn initialize_commands(registry: &mut CommandRegistry) {
    // Each command registers itself
    RenewCommand::register_with_registry(registry);
    SaveFilesCommand::register_with_registry(registry);
    SaveLastAnswerCommand::register_with_registry(registry);
}
