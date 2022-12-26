//! A Hue bridge

use super::lights::LightState;
use crate::config::*;
use crate::error::AppError;
use colored::Colorize;
use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Record, RecordKind};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::Write;
use std::{collections::HashMap, net::IpAddr, time::Duration};
use tokio::time::timeout;
use tracing::info;

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
/// A Hue Bridge client providing API commands
///
#[derive(Debug)]
pub struct Bridge {
    pub ip_address: IpAddr,
    pub auth_key: String,
    //     pub config_info: ConfigInfo,
    //     pub client: reqwest::Client,
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

        let cfg = get_app_cfg();
        let ip_address = cfg.bridge_ipaddr.unwrap();
        let auth_key = cfg.auth_key.unwrap();

        Self {
            ip_address,
            auth_key,
        }
    }

    /// Gets an endpoint response and deserialises it.
    #[tracing::instrument(skip(self))]
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, AppError> {
        let cfg = get_app_cfg();
        // TODO check auth_key is valid and remove unwrap below
        let url = format!(
            "http://{}/api/{}/{}",
            self.ip_address,
            cfg.auth_key.unwrap(),
            endpoint
        );

        info!(url, "fetching");

        let client = reqwest::Client::builder()
            // .timeout(Duration::new(5, 0))
            .build()
            .unwrap();

        tracing::debug!("starting get");
        let resp = client
            .get(url)
            .timeout(std::time::Duration::from_millis(500))
            .send()
            .await
            .map_err(|_| AppError::NetworkError)
            .unwrap();
        tracing::debug!("ending get");

        let status = resp.status();

        match status {
            StatusCode::OK => Ok(resp.json().await.map_err(|_| AppError::Other)?),
            StatusCode::NOT_FOUND => Err(AppError::APINotFound),
            _ => Err(AppError::Other),
        }
    }

    /// Puts the given data to the given endpoint
    pub async fn put(&self, endpoint: &str, data: &LightState) -> Result<(), AppError> {
        let cfg = get_app_cfg();
        // TODO check auth_key is valid and remove unwrap below
        let url = format!(
            "http://{}/api/{}/{}",
            self.ip_address,
            cfg.auth_key.unwrap(),
            endpoint
        );
        let client = reqwest::Client::new();
        let body_json = serde_json::to_string(data).unwrap();
        let resp = client.put(&url).body(body_json).send().await.unwrap();

        match resp.status() {
            StatusCode::OK => Ok(()),
            StatusCode::NOT_FOUND => Err(AppError::APINotFound),
            _ => Err(AppError::Other),
        }
    }
}

/// Return true if config file contains an ip address and auth key.
fn is_configured() -> bool {
    let cfg = get_app_cfg();
    cfg.bridge_ipaddr.is_some() && cfg.auth_key.is_some()
}

/// Gets the IP address, creates an auth_key, and saves both to the config file.
async fn configure() -> Result<(), AppError> {
    let mut cfg = get_app_cfg();

    let ipaddr = match get_bridge_ipaddr().await {
        Ok(ipaddr) => ipaddr,
        Err(err) => return Err(err),
    };

    let auth_key = match create_new_auth_key(ipaddr).await {
        Ok(auth_key) => auth_key,
        Err(err) => return Err(err),
    };

    cfg.bridge_ipaddr = Some(ipaddr);
    cfg.auth_key = Some(auth_key);

    store_app_cfg(&cfg);

    Ok(())
}
/// Gets the Hue Bridge ip address.
pub async fn get_bridge_ipaddr() -> Result<IpAddr, AppError> {
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
                let addr = response.records().filter_map(self::to_ip_addr).next();
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
pub async fn create_new_auth_key(ip_addr: IpAddr) -> Result<String, AppError> {
    let url = format!("http://{}/api", ip_addr);
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

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}
