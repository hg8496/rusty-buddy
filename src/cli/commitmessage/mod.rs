mod cm_args;
mod run;

pub use cm_args::CommitMessageArgs;

pub async fn run(_args: CommitMessageArgs) -> Result<(), Box<dyn std::error::Error>> {
    run::run_commitmessage().await
}
