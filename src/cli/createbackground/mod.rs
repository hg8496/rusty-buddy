mod bg_args;
mod run;

pub use bg_args::{BackgroundArgs, Orientation};

pub async fn run(args: BackgroundArgs) -> Result<(), Box<dyn std::error::Error>> {
    run::run_create_background(args.file.as_str(), args.orientation).await
}
