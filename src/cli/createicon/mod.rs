mod ci_args;
pub mod createicon;

pub use ci_args::CreateIconArgs;

pub async fn run(args: CreateIconArgs) -> Result<(), Box<dyn std::error::Error>> {
    crate::cli::createicon::createicon::run_createicon(args.output.as_str(), args.sizes).await
}
