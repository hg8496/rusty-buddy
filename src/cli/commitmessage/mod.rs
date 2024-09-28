mod cm_args;
pub mod commitmessage;

pub use cm_args::CommitMessageArgs;

pub async fn run(_args: CommitMessageArgs) -> Result<(), Box<dyn std::error::Error>> {
    commitmessage::run_commitmessage().await
}
