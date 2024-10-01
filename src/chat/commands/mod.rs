use crate::chat::command::RegisterableCommand;
use crate::chat::command_registry::CommandRegistry;
use crate::chat::commands::refresh::RenewCommand;
use crate::chat::commands::save_files::SaveFilesCommand;

mod refresh;
mod save_files;

pub fn initialize_commands(registry: &mut CommandRegistry) {
    // Each command registers itself
    RenewCommand::register_with_registry(registry);
    SaveFilesCommand::register_with_registry(registry);
}
