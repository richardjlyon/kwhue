/// Handles app configuration
///
use crate::error::AppError;
use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Record, RecordKind};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::net::SocketAddr;
use std::{collections::HashMap, time::Duration};
use tokio::time::timeout;

const CONFIG_NAME: &str = "kwhue";

// #[derive(Serialize, Deserialize, Default)]
// pub struct AppConfig {
//     pub auth_key: Option<String>,
//     pub bridge_ipaddr: Option<IpAddr>,
// }

// NO IP, KEY <-- NO!

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthAppConfig {
    pub ip: SocketAddr,
    pub key: String,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize, Debug)]
pub struct ConfigInfo {
    #[serde(rename = "bridgeid")]
    pub bridge_id: String,
    pub apiversion: String,
    pub swversion: String,
    pub ipaddress: String,
}

impl ConfigInfo {
    pub fn software_version(&self) -> String {
        // 1.55.0 -> 1.55
        let parts: Vec<&str> = self.apiversion.split(".").collect();
        format!("{}.{}.{}", parts[0], parts[1], self.swversion)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum AppConfig {
    Uninit,
    Unauth { ip: SocketAddr },
    Auth(AuthAppConfig),
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

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum BridgeStatus {
    CONNECTED,
    DISCONNECTED,
}

////////////////////////////////////////////////////////////////////////////////

/// Bridge authorisation key data
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthKeyResponse {
    pub username: String,
}

////////////////////////////////////////////////////////////////////////////////

/// Bridge error data
#[derive(Deserialize, Debug)]
struct BasicError {
    #[serde(rename = "type")]
    error_type: u32,
    description: String,
}

/// Represents a response from the bridge
///
/// json example
///
/// { "error": {} } // Error<T>
/// { "success": {} } // Success<T>
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Response<T, E> {
    Error(Error<E>),
    Success(Success<T>),
}

/// Bridge 'error' response
#[derive(Deserialize, Debug)]
struct Error<T> {
    error: T,
}

/// Bridge 'sucess' response
#[derive(Deserialize, Debug)]
struct Success<T> {
    success: T,
}

////////////////////////////////////////////////////////////////////////////////

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

/// Return true if config file contains an ip address and auth key.
pub fn is_configured() -> bool {
    get_app_cfg().is_configured()
}

/// Gets the IP address, creates an auth_key, and saves both to the config file.
pub async fn configure() -> Result<AuthAppConfig, AppError> {
    let ipaddr = get_bridge_socket().await?;
    let auth_key = create_new_auth_key(ipaddr).await?;

    let cfg = AuthAppConfig {
        ip: ipaddr,
        key: auth_key,
    };

    store_app_cfg(&AppConfig::Auth(cfg.clone()));

    Ok(cfg)
}
/// Gets the Hue Bridge ip address.
pub async fn get_bridge_socket() -> Result<SocketAddr, AppError> {
    const SERVICE_NAME: &str = "_hue._tcp.local";

    let responses = mdns::discover::all(SERVICE_NAME, Duration::from_secs(1))
        .map_err(|_| AppError::Other)?
        .listen();

    pin_mut!(responses);

    // if the hub is disconnected, it will timeout and block the app
    let response = match timeout(Duration::from_secs(5), responses.next()).await {
        Ok(r) => Ok(r),
        Err(_) => Err(AppError::HueBridgeTimeout),
    };

    match response {
        Ok(r) => match r {
            Some(Ok(response)) => {
                let addr = response.records().filter_map(self::to_socket_addr).next();
                if let Some(addr) = addr {
                    Ok(addr)
                } else {
                    Err(AppError::HueBridgeAddressNotFoundError)
                }
            }
            Some(Err(_)) => todo!(),
            None => Err(AppError::Other),
        },
        Err(err) => Err(err),
    }
}

/// Creates a hub authorisation key.
///
/// See [Hue Configuration API](https://developers.meethue.com/develop/hue-api/7-configuration-api/)
pub async fn create_new_auth_key(socket_addr: SocketAddr) -> Result<String, AppError> {
    let url = format!("http://{}/api", socket_addr);
    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("devicetype", "kwhue_app rust_app");

    print!("Press link button");
    std::io::stdout().flush().unwrap();

    let response = loop {
        let resp = client.post(&url).json(&params).send().await.unwrap();
        let status = resp.status();

        let mut data: Vec<Response<AuthKeyResponse, BasicError>> = match status {
            StatusCode::OK => resp.json().await.map_err(|err| {
                println!("error: {}", err);
                AppError::HueBridgeAuthKeyInvalid
            }),
            StatusCode::NOT_FOUND => Err(AppError::APINotFound),
            _ => Err(AppError::Other),
        }
        .unwrap();

        // the hub returns an error until the link button is pressed
        match data.pop().unwrap() {
            Response::Error(e) => {
                if e.error.error_type != 101 {
                    println!("Error: {}", e.error.description);
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
                print!(".");
                std::io::stdout().flush().unwrap();
            }
            Response::Success(s) => break s.success,
        }
    };

    Ok(response.username)
}

///
pub async fn bridge_status() -> BridgeStatus {
    let result = get_bridge_socket().await;
    match result {
        Ok(_) => BridgeStatus::CONNECTED,
        Err(_) => BridgeStatus::DISCONNECTED,
    }
}

fn to_socket_addr(record: &Record) -> Option<SocketAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(SocketAddr::new(addr.into(), 80)),
        RecordKind::AAAA(addr) => Some(SocketAddr::new(addr.into(), 80)),
        _ => None,
    }
}
