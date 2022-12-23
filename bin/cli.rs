use clap::Parser;
use kwhue::cli::{commands, Cli, Commands};
use kwhue::error::AppError;
use kwhue::hue::bridge::Bridge;

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    let bridge = Bridge::new().await;

    match &cli.command {
        Commands::List {} => commands::lights::all(&bridge).await,
        Commands::Toggle { id } => commands::lights::toggle(&bridge, id).await,
    }

    // bridge.new_user().await;

    Ok(())
}
