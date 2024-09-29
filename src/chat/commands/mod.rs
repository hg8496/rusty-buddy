use crate::chat::command::RegisterableCommand;
use crate::chat::command_registry::CommandRegistry;
use crate::chat::commands::refresh::RenewCommand;

mod refresh;

pub fn initialize_commands(registry: &mut CommandRegistry) {
    // Each command registers itself
    RenewCommand::register_with_registry(registry);
    // Add other commands here as needed
}
