/// Handles app configuration
///
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

const CONFIG_NAME: &str = "kwhue";

// #[derive(Serialize, Deserialize, Default)]
// pub struct AppConfig {
//     pub auth_key: Option<String>,
//     pub bridge_ipaddr: Option<IpAddr>,
// }

// NO IP, KEY <-- NO!

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum AppConfig {
    Uninit,
    Unauth { ip: SocketAddr },
    Auth(AuthAppConfig),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthAppConfig {
    pub ip: SocketAddr,
    pub key: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig::Uninit
    }
}

impl AppConfig {
    pub fn is_configured(&self) -> bool {
        match self {
            AppConfig::Uninit => false,
            AppConfig::Unauth { .. } => false,
            AppConfig::Auth { .. } => true,
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
