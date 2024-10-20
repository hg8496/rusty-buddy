//! This module provides functionality for managing knowledge entries for various commands.        
//! It allows users to store, retrieve, and delete knowledge entries that enhance the interactivity
//! of whether it's for wishes or chats.                                                           

mod add;
mod init;
mod knowledge_args;
mod run;
mod search;

pub use knowledge_args::KnowledgeArgs;
pub use run::run_knowledge;
