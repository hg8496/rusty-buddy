pub mod chat;
mod chat_args;

pub use chat_args::ChatArgs;

pub async fn run(args: ChatArgs) -> Result<(), Box<dyn std::error::Error>> {
    chat::run_chat(
        args.new,
        args.continue_last,
        args.load,
        args.directory,
        args.persona,
    )
    .await
}
