//! This module defines command-line arguments for generating backgrounds using the `clap` crate.
//! It provides the `BackgroundArgs` struct, which holds the parameters necessary for creating
//! a background image based on user input. The module supports specifying the output file path
//! and the orientation of the generated background, enhancing user control over the background
//! creation process.
//!
//! ## Structure
//!
//! The `BackgroundArgs` struct is used to parse and store arguments provided by the user
//! when running the background generation program. It includes the output file for the generated
//! backgrounds and the orientation of the background, which can either be landscape or portrait.
//!
//! ### Fields
//!
//! - `file`: A string representing the output file for the generated backgrounds, with a
//!   default value of `"./background.png"`.
//! - `orientation`: An enumeration that indicates the desired orientation of the background,
//!   which can be either `Landscape` or `Portrait`.
//!
//! ## Example
//!
//! To use the `BackgroundArgs` struct in context, you might invoke it as follows:
//!
//! ```rust
//! use clap::Parser;
//!
//! #[derive(Parser)]
//! struct Cli {
//!     #[command(subcommand)]
//!     background: BackgroundArgs,
//! }
//!
//! let args = Cli::parse();
//! println!("Output file: {}", args.background.file);
//! println!("Orientation: {:?}", args.background.orientation);
//! ```
//!
//! This example demonstrates how to leverage the `BackgroundArgs` struct to obtain user-defined
//! preferences for background generation. Users can easily dictate details such as output file
//! path and image geometry via command line arguments.

use clap::{Args, ValueEnum};

#[derive(Args)]
pub struct BackgroundArgs {
    /// Output directory for the generated backgrounds
    #[arg(short, long, default_value = "./background.png")]
    pub file: String,

    /// Orientation of the background
    #[arg(short, long, value_enum)]
    pub orientation: Orientation,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Orientation {
    Landscape,
    Portrait,
}
