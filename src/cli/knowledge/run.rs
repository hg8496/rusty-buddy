//! This module provides functionality for managing knowledge entries based
//! on user-defined commands within the Rusty Buddy application.

use crate::cli::knowledge::{add, init, search, KnowledgeArgs};
use std::error::Error;

pub async fn run_knowledge(args: KnowledgeArgs) -> Result<(), Box<dyn Error>> {
    match args {
        KnowledgeArgs::Init(init) => {
            init::init(init).await?;
        }
        KnowledgeArgs::Search(search) => {
            search::search(search).await?;
        }
        KnowledgeArgs::Add(add) => {
            add::add(add).await?;
        }
    }

    Ok(())
}
