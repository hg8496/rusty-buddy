//! This module defines the `KnowledgeArgs` struct, which holds the command-line arguments
//! related to managing knowledge entries in the Rusty Buddy application.
//!
//! It utilizes the `clap` library for parsing, providing a clear interface for users specifying
//! how to manage knowledge entries.

use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum KnowledgeArgs {
    Init(InitArgs),
    Search(SearchArgs),
    Add(AddArgs), // New subcommand for adding knowledge
}

#[derive(Args)]
pub struct SearchArgs {
    pub search: String,
}

#[derive(Args)]
pub struct InitArgs {
    /// Specify a persona for the knowledge initialization
    #[arg(short, long)]
    pub persona: Option<String>,
}

#[derive(Args)] // New AddArgs struct for the add subcommand
pub struct AddArgs {
    /// Add a directory of files to the knowledge database
    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    pub dir: Option<PathBuf>,

    /// Add a single file to the knowledge database
    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    pub file: Option<PathBuf>,

    /// Add a webpage to the knowledge database
    #[arg(short, long)]
    pub url: Option<String>,
}
