use clap::Args;

#[derive(Args)]
pub struct CreateIconArgs {
    /// Output directory for the generated icons
    #[arg(short, long, default_value = "./icons")]
    pub output: String,

    /// Comma-separated list of icon sizes to generate
    #[arg(short, long, default_values_t = [16,32,64,128,256,512])]
    pub sizes: Vec<u32>,
}
