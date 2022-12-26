use clap::Parser;
use kwhue::cli::{commands::*, AdminSubcommands, Cli, Commands, LightSubcommands};
use kwhue::error::AppError;
use kwhue::hue::bridge::Bridge;

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();

    let bridge = Bridge::new().await;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Admin(u) => match u {
            AdminSubcommands::Reset {} => admin::reset().await,
            AdminSubcommands::Check {} => admin::info(&bridge).await,
        },
        Commands::Light(l) => match l {
            LightSubcommands::List {} => light::all(&bridge).await,
            LightSubcommands::Toggle { id } => light::toggle(&bridge, id).await,
            LightSubcommands::On { id } => light::on(&bridge, id).await,
            LightSubcommands::Off { id } => light::off(&bridge, id).await,
        },
    };

    Ok(())
}
