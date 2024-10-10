//! This module provides functionality for managing knowledge entries based
//! on user-defined commands within the Rusty Buddy application.

use crate::cli::knowledge::{init, search, KnowledgeArgs};
use std::error::Error;

/// Runs the knowledge command, executing the specified action based on user input.
///
/// # Arguments
///
/// * `args` - KnowledgeArgs holding the parameters for the command.
///
/// # Returns
///
/// Returns a Result indicating success or an error if the process fails.
pub async fn run_knowledge(args: KnowledgeArgs) -> Result<(), Box<dyn Error>> {
    match args {
        KnowledgeArgs::Init(init) => {
            init::init(init).await?;
        }
        KnowledgeArgs::Search(search) => {
            search::search(search).await?;
        }
    }

    Ok(())
}
