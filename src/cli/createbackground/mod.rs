mod bg_args;
pub mod createbackground;

pub use bg_args::{BackgroundArgs, Orientation};

pub async fn run(args: BackgroundArgs) -> Result<(), Box<dyn std::error::Error>> {
    createbackground::run_create_background(args.file.as_str(), args.orientation).await
}
