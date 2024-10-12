//! This module defines the `KnowledgeArgs` struct, which holds the command-line arguments
//! related to managing knowledge entries in the Rusty Buddy application.
//!
//! It utilizes the `clap` library for parsing, providing a clear interface for users specifying
//! how to manage knowledge entries.

use clap::{Args, Subcommand};

/// This struct holds the command-line arguments for managing knowledge entries.
#[derive(Subcommand)]
pub enum KnowledgeArgs {
    Init(InitArgs),
    Search(SearchArgs),
}

#[derive(Args)]
pub struct SearchArgs {
    pub search: String,
}

#[derive(Args)]
pub struct InitArgs {
    /// Specify a persona for the knowledge initalisation
    #[arg(short, long)]
    pub persona: Option<String>,
}
