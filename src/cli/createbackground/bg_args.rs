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
