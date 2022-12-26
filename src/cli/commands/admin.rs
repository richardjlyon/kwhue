//! admin commands
//!
//! > admin init
//! > admin check
//!
use crate::hue::bridge::bridge_status;
use crate::hue::bridge::BridgeStatus;
use crate::{
    config::{store_app_cfg, AppConfig},
    hue::bridge::Bridge,
};
use colored::Colorize;
/// Reset the app.
///
/// Clearing the config file forces it to reinitialise next time it starts up.
pub async fn reset() {
    // reset config file
    let cfg = AppConfig::default();
    store_app_cfg(&cfg);

    println!(
        "{}: {}",
        "info".green().bold(),
        "app reset, restart app to reinitialse".bold()
    );
}

/// Print the status of the Hue bridge to the terminal
pub async fn info(bridge: &Bridge) {
    // connection status
    println!("{}", "Status".bold());

    match bridge_status().await {
        BridgeStatus::CONNECTED => println!("{}\n", "Connected".green()),
        BridgeStatus::DISCONNECTED => {
            println!("{}", "Disconnected".red());
            std::process::exit(1);
        }
    }

    match bridge.config_info().await {
        Ok(data) => {
            println!("{}", "ID".bold());
            println!("{}\n", data.bridge_id);

            println!("{}", "Software".bold());
            println!("{}\n", data.software_version());

            println!("{}", "IP address".bold());
            println!("{}\n", data.ipaddress);
        }
        Err(_) => todo!(),
    }
}
