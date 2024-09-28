use clap::Args;

#[derive(Args)]
pub struct WishArgs {
    /// The directory to collect files from
    pub directory: String,

    /// Activate the usage of tools
    #[arg(short, long)]
    pub tools: bool,
}
