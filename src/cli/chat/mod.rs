mod chat_args;
mod run;

pub use chat_args::ChatArgs;

pub async fn run(args: ChatArgs) -> Result<(), Box<dyn std::error::Error>> {
    run::run_chat(args).await
}
