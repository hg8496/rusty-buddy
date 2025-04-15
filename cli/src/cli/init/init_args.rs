use clap::Args;

#[derive(Args)]
pub struct InitArgs {
    #[arg(long)]
    pub choose_persona: bool,
}
