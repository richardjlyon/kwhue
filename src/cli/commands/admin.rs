//! admin commands
//!
//! > admin init
//! > admin check
//!
use crate::hue::config_info::config_info;
use crate::{
    config::{get_app_cfg, store_app_cfg, AppConfig},
    error::AppError,
    hue::bridge::{get_bridge_ipaddr, Bridge},
};
use colored::Colorize;
use std::io::Write;

/// Reset the app
///
/// Gets the Hue bridge ip address, checks it is configured properly, obtains
/// an authorisation key, and stores the results in the configuration file.
///   
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
///
/// Checks the values in the config file, and tests that the bridge responds to
/// the given IP address if specified
///
pub async fn check(bridge: &Bridge) {
    let mut cfg = get_app_cfg();

    // Checks Hue bridge ip address is stored, gets and stores it if not, and
    // exits the program with an error message if not

    if cfg.bridge_ipaddr.is_none() {
        print!("{}", "Bridge IP address not set, searching...".yellow());
        std::io::stdout().flush().unwrap();

        match get_bridge_ipaddr().await {
            Ok(addr) => {
                println!("{}", "found".yellow());
                cfg.bridge_ipaddr = Some(addr);
                store_app_cfg(&cfg);
            }
            Err(AppError::NetworkError) => {
                println!("{}", "Error. Network problem. Exiting.".red());
                std::process::exit(1);
            }
            Err(AppError::HueBridgeNotFoundError) => {
                println!("{}", "Error. Hue bridge not found. Exiting.".red());
                std::process::exit(1);
            }
            Err(_) => {
                println!(
                    "{}",
                    "Something went wrong, but I don't know what. Exiting.".red()
                );
                std::process::exit(1);
            }
        }
    }

    // Checks the Hue bridge ip address responds and the brudge returns its
    // configuration info, and exits the program with an error message if not

    if cfg.bridge_ipaddr.is_some() {
        println!(
            "{} {}",
            "Bridge IP address".green(),
            cfg.bridge_ipaddr.unwrap().to_string().green().bold()
        );

        print!("{}", "Checking bridge...".yellow());
        std::io::stdout().flush().unwrap();

        match config_info(&cfg.bridge_ipaddr.unwrap()).await {
            Ok(info) => {
                println!("{}", "OK".yellow());
                println!("{} {}", "Bridge ID".green(), info.bridge_id.green());
            }
            Err(AppError::HueBridgeTimeout) => {
                println!();
                println!(
                    "{}: {}",
                    "error".red().bold(),
                    AppError::HueBridgeTimeout.to_string().bold()
                );
                std::process::exit(1);
            }
            Err(AppError::HueBridgeMisconfigured) => {
                println!();
                println!(
                    "{}: {}",
                    "error".red().bold(),
                    AppError::HueBridgeMisconfigured.to_string().bold()
                );
                std::process::exit(1);
            }
            Err(_) => {}
        }
    }

    // check authorisation

    if cfg.auth_key.is_none() {
        println!("{}", "Bridge auth key not set...".yellow());
    }

    // let config_filepath = get_cfg_file_path();
    // let ip_addr = match cfg.bridge_ipaddr {
    //     Some(addr) => addr.to_string(),
    //     None => String::from("Not set"),
    // };
    // println!("Bridge IP Address : {}", ip_addr);
    // println!(
    //     "Auth key          : {}",
    //     cfg.auth_key.unwrap_or(String::from("Not set"))
    // );
    // println!("Config file       : {}", config_filepath);

    // TODO save cfg file
}
