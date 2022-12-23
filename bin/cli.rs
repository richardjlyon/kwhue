use clap::Parser;
use kwhue::cli::{commands::lights, AdminSubcommands, Cli, Commands, LightSubcommands};
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
            AdminSubcommands::User {} => bridge.new_user().await,
        },
        Commands::Light(l) => match l {
            LightSubcommands::List {} => lights::all(&bridge).await,
            LightSubcommands::Toggle { id } => lights::toggle(&bridge, id).await,
            LightSubcommands::On { id } => lights::on(&bridge, id).await,
            LightSubcommands::Off { id } => lights::off(&bridge, id).await,
        },
    };

    Ok(())
}
