pub mod wish;
mod wish_args;

pub use wish_args::WishArgs;


pub async fn run(args: WishArgs) -> Result<(), Box<dyn std::error::Error>> {
    wish::run_wish(args.directory.as_str(), args.tools).await
}

