mod chat_args;
mod run;

pub use chat_args::ChatArgs;

pub async fn run(args: ChatArgs) -> Result<(), Box<dyn std::error::Error>> {
    run::run_chat(
        args.new,
        args.continue_last,
        args.load,
        args.directory,
        args.persona,
        args.one_shot,
        args.message,
    )
    .await
}
