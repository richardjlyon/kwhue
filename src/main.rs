use clap::Parser;
use kwhue::cli::{commands, Cli, Commands};
use kwhue::error::AppError;
use kwhue::hue::bridge::Bridge;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    let bridge = Bridge::new().await;

    match &cli.command {
        Commands::Lights {} => commands::lights::all(&bridge).await,
    }

    // bridge.new_user().await;

    Ok(())
}
