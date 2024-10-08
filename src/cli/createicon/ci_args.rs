//! This module defines command-line argument parsing for generating icons using the `clap` crate.
//! It provides structured data to hold the output directory and sizes of icons to be generated.
//!
//! The `CreateIconArgs` struct is populated from command line arguments, allowing users to specify
//! an output directory and a list of icon sizes. Default values are provided for the output directory
//! and sizes if not specified by the user.
//!
//! ## Usage Example
//!
//! Here's an example of how you might specify options for generating icons using the `CreateIconArgs`
//! struct:
//!
//! ```rust
//! use crate::cli::createicon::CreateIconArgs;
//! use clap::Parser;
//!
//! let args = CreateIconArgs::parse(); // Parses command line arguments
//! // Use args.output and args.sizes as needed
//! ```
//!
//! ### Fields
//!
//! - `output`: A string specifying the output directory for the generated icons. The default value is `"./icons"`.
//! - `sizes`: A vector of unsigned integers representing the comma-separated list of icon sizes to generate.
//!   Default values are `[16, 32, 64, 128, 256, 512]`.
//!
use clap::Args;

/// This struct holds the command-line arguments for creating icons.
#[derive(Args)]
pub struct CreateIconArgs {
    /// Output directory for the generated icons
    #[arg(short, long, default_value = "./icons")]
    pub output: String,

    /// Comma-separated list of icon sizes to generate
    #[arg(short, long, default_values_t = [16,32,64,128,256,512])]
    pub sizes: Vec<u32>,
}
