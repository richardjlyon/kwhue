//! A Hue bridge

use crate::config::*;
use crate::error::AppError;
use colored::Colorize;
use futures_util::{pin_mut, stream::StreamExt};
use lights::LightState;
use mdns::{Record, RecordKind};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::Write;
use std::{collections::HashMap, net::SocketAddr, net::SocketAddrV6, time::Duration};
use tokio::time::timeout;
use tracing::trace;

pub mod api;
pub mod lights;

/// A Hue Bridge client providing API commands
///
#[derive(Debug)]
pub struct Bridge {
    pub config: AppConfig,
    pub client: reqwest::Client,
    //     pub config_info: ConfigInfo,
}

impl Bridge {
    /// Create a new Hue bridge instance
    pub async fn new() -> Self {
        if !is_configured() {
            match configure().await {
                Ok(_) => {}
                Err(err) => {
                    println!("{}: {}\n", "error".red().bold(), err.to_string().bold());
                    std::process::exit(1);
                }
            }
        }

        Self::new_with_config(get_app_cfg())
    }

    pub fn new_with_config(config: AppConfig) -> Self {
        let client = reqwest::Client::builder().build().unwrap();

        Self { config, client }
    }

    /// Gets an endpoint response and deserialises it.
    #[tracing::instrument(skip(self))]
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, AppError> {
        let url = self.get_url(endpoint)?;

        trace!(url, "fetching");

        let resp = self
            .client
            .get(url)
            .timeout(std::time::Duration::from_millis(500))
            .send()
            // we have a From impl from reqwest::Error to AppError
            // so we can use the ? operator to convert the error automatically
            .await?;

        let status = resp.status();

        match status {
            StatusCode::OK => Ok(resp.json().await?),
            StatusCode::NOT_FOUND => Err(AppError::APINotFound),
            _ => Err(AppError::Other),
        }
    }

    /// Puts the given data to the given endpoint
    pub async fn put(&self, endpoint: &str, data: &LightState) -> Result<(), AppError> {
        let url = self.get_url(endpoint)?;
        let body_json = serde_json::to_string(data).unwrap();
        let resp = self.client.put(&url).body(body_json).send().await.unwrap();

        match resp.status() {
            StatusCode::OK => Ok(()),
            StatusCode::NOT_FOUND => Err(AppError::APINotFound),
            _ => Err(AppError::Other),
        }
    }

    pub async fn config_info(&self) -> Result<ConfigInfo, AppError> {
        let data: ConfigInfo = self.get("config").await?;

        Ok(data)
    }

    fn get_url(&self, endpoint: &str) -> Result<String, AppError> {
        // if we match on the object, the internals are moved into the match statement
        // this allows you to do things like
        // let url = match option {
        //     Some(s) => s,
        //     None => todo!(),
        // }
        // now url 'owns' the data that was in the option variable
        match &self.config {
            AppConfig::Uninit => Err(AppError::Other),
            AppConfig::Unauth { ip } => Ok(format!("http://{}/api/{}", ip, endpoint)),
            AppConfig::Auth(AuthAppConfig { ip, key }) => {
                Ok(format!("http://{}/api/{}/{}", ip, key, endpoint))
            }
        }
    }
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

/// Bridge authorisation key data
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthKeyResponse {
    pub username: String,
}

/// Bridge error data
#[derive(Deserialize, Debug)]
struct BasicError {
    #[serde(rename = "type")]
    error_type: u32,
    description: String,
}

#[derive(Debug)]
pub enum BridgeStatus {
    CONNECTED,
    DISCONNECTED,
}

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

/// Return true if config file contains an ip address and auth key.
fn is_configured() -> bool {
    get_app_cfg().is_configured()
}

/// Gets the IP address, creates an auth_key, and saves both to the config file.
async fn configure() -> Result<AuthAppConfig, AppError> {
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

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn gets_lights() {
        // create a mock server
        let mock = httpmock::MockServer::start_async().await;

        let test_response = include_str!("test_data/lights_respon.json");

        // set up handlers for specific requests
        let get_lights = mock
            .mock_async(|when, then| {
                when.method("GET").path("/api/auth/lights");
                then.status(200).body(test_response);
            })
            .await;

        // set up the bridge with the mock server's ip + port (socket addr)
        let bridge = Bridge::new_with_config(AppConfig::Auth(AuthAppConfig {
            key: "auth".to_string(),
            ip: mock.address().to_owned(),
            // ip: SocketAddr::V4(SocketAddrV4::new(std::net::Ipv4Addr::new(10, 1, 1, 1), 80)),
        }));

        // make a request
        let lights = bridge.get_lights().await.unwrap();

        // ensure that the api is called exactly once
        get_lights.assert_hits_async(1).await;

        let times_requested = get_lights.hits();
        println!("this endpoint was hit {} times", times_requested);
    }
}
