//! This module defines command-line arguments for the wish application using the Clap library.
//!
//! It includes a mandatory `directory` argument for specifying the source directory,
//! and an optional `tools` flag to activate usage of tools, which enable the application
//! to fulfill user wishes such as file creation or modification based on the described tasks.
//!
//! ## Structure
//!
//! The `WishArgs` struct is used to parse and store arguments provided by the user
//! when running the wish generation program. This allows for flexible command usage
//! while ensuring users provide the necessary input for the operation.
//!
//! # Example Usage
//!
//! Here's how you might define command-line arguments using `WishArgs`:
//!
//! ```rust
//! use crate::cli::wish::WishArgs;
//! use clap::Parser;
//!
//! let args = WishArgs::parse();
//! println!("Directory specified: {:?}", args.directory);
//! println!("Tools enabled: {}", args.tools);
//! ```
//!
//! ## Fields
//!
//! - `directory`: A vector of `PathBuf` representing the source directory from which files will be collected.
//! - `tools`: A boolean flag that indicates whether the application should use tools to fulfill user wishes,
//!   which may involve creating or modifying files and directories based on user-specified actions.

use clap::Args;
use std::path::PathBuf;

/// This struct defines command-line arguments for a wish application using the Clap library.
/// It includes a mandatory `directory` argument for specifying the source directory,
/// and an optional `tools` flag to activate the usage of tools.
#[derive(Args)]
pub struct WishArgs {
    /// Directories to add to the chat context
    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    pub directory: Option<Vec<PathBuf>>,

    /// Activate the usage of tools
    #[arg(short, long)]
    pub tools: bool,
}
