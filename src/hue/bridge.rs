use crate::config::*;
use crate::error::AppError;
use colored::Colorize;
use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Record, RecordKind};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize};
use std::{net::IpAddr, time::Duration};
use tokio::time::timeout;
use tracing::{info, trace};

use simple_mdns::async_discovery::OneShotMdnsResolver;

use super::lights::LightState;

/// A Hue Bridge client providing API commands
///
#[derive(Debug)]
pub struct Bridge {
    pub ip_address: IpAddr,
    pub auth_key: String,
    //     pub config_info: ConfigInfo,
    //     pub client: reqwest::Client,
}

/// Hue configuration information
///
#[derive(Deserialize, Debug)]
pub struct ConfigInfo {
    #[serde(rename = "bridgeid")]
    pub bridge_id: String,
}

/// Create a new Hue bridge instance
///
impl Bridge {
    pub async fn new() -> Self {
        let cfg = get_app_cfg();

        if cfg.bridge_ipaddr.is_none() {
            println!("{}: {}", "warning".yellow(), "app not initialised");
            crate::cli::commands::admin::init().await;
        }

        let cfg = get_app_cfg();
        let ip_address = cfg.bridge_ipaddr.unwrap();

        // let config_info = config_info(&ip_address).await.unwrap();
        // let client = reqwest::Client::builder()
        //     .timeout(Duration::new(5, 0))
        //     .build()
        //     .unwrap();

        Self {
            ip_address,
            // config_info,
            // client,
        }
    }
}

/// Get an endpoint
///
impl Bridge {
    /// get the given endpoint
    ///
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

    /// put to the given endpoint
    ///
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

/// get the Hue Bridge ip address
///
pub async fn get_bridge_ipaddr() -> Result<IpAddr, AppError> {
    const SERVICE_NAME: &str = "_hue._tcp.local";

    tracing::info!("discovering...");
    let responses = mdns::discover::all(SERVICE_NAME, Duration::from_secs(1))
        .map_err(|_| AppError::Other)?
        .listen();
    pin_mut!(responses);

    match responses.next().await {
        Some(Ok(response)) => {
            let addr = response.records().filter_map(self::to_ip_addr).next();
            if let Some(addr) = addr {
                println!("Got address {:?}", addr);
                Ok(addr)
            } else {
                Err(AppError::HueBridgeAddressNotFoundError)
            }
        }
        Some(Err(_)) => Err(AppError::Other),
        None => Err(AppError::HueBridgeNotFoundError),
    }
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}

/// Reads the Hue bridge configuration info
///
pub async fn config_info(ip_address: &IpAddr) -> Result<ConfigInfo, AppError> {
    let client = reqwest::Client::builder().build().unwrap();
    let url = format!("http://{}/api/0/config", ip_address);
    let resp = client
        .get(url)
        .timeout(std::time::Duration::from_millis(5000))
        .send()
        .await;

    // intercept timeout due to bad ip address
    let resp = match resp {
        Ok(resp) => resp,
        Err(err) => {
            if err.is_timeout() {
                return Err(AppError::HueBridgeTimeout);
            } else {
                return Err(AppError::Other);
            }
        }
    };

    // intercept misconfiguration and return configuration info
    let text = resp.text().await.expect("couldn't get text");
    match serde_json::from_str(&text) {
        Ok(config_info) => Ok(config_info),
        Err(_) => Err(AppError::HueBridgeMisconfigured),
    }
}
