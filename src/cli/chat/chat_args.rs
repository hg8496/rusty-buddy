use clap::Args;

#[derive(Args)]
pub struct ChatArgs {
    /// Start a new chat session
    #[arg(short, long)]
    pub new: bool,

    /// Continue the last chat session
    #[arg(short, long, action)]
    pub continue_last: bool,

    /// Load a specific chat session by name
    #[arg(short, long)]
    pub load: Option<String>,

    /// Directory to add to the chat context
    #[arg(short, long)]
    pub directory: Option<String>,

    /// Specify a persona for the chat session
    #[arg(short, long)]
    pub persona: Option<String>,
}
