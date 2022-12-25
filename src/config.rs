/// Handles app configuration
///
use crate::hue::new_user::AuthKeyResponse;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

const CONFIG_NAME: &str = "kwhue";

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub auth_key: Option<String>,
    pub bridge_ipaddr: Option<IpAddr>,
}

impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        Self {
            auth_key: None,
            bridge_ipaddr: None,
        }
    }
}

/// Get the user configuration data
pub fn get_app_cfg() -> AppConfig {
    confy::load(CONFIG_NAME, None).unwrap()
}

/// Store the user configuration data
pub fn store_app_cfg(cfg: &AppConfig) {
    confy::store(CONFIG_NAME, None, cfg).unwrap();
}

/// Get the user configuration data file path
pub fn get_cfg_file_path() -> String {
    confy::get_configuration_file_path(CONFIG_NAME, None)
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}
