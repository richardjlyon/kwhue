use crate::error::AppError;
use crate::get_user_cfg;
use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Record, RecordKind};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize};
use std::{net::IpAddr, time::Duration};
use tracing::info;

/// A Hue Bridge client providing API commands
///
#[derive(Debug)]
pub struct Bridge {
    pub ip_address: IpAddr,
    pub config_info: ConfigInfo,
}

/// Hue configuration information
///
#[derive(Deserialize, Debug)]
pub struct ConfigInfo {
    #[serde(rename = "bridgeid")]
    pub bridge_id: String,
}

/// Create a new instance
///
impl Bridge {
    pub async fn new() -> Self {
        let ip_address = bridge_ipaddr().await.unwrap();
        let config_info = config_info(&ip_address).await.unwrap();

        Self {
            ip_address,
            config_info,
        }
    }
}

/// Get an endpoint
///
impl Bridge {
    /// get the given endpoint
    #[tracing::instrument(skip(self))]
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, AppError> {
        let cfg = get_user_cfg();
        let url = format!(
            "http://{}/api/{}/{}",
            self.ip_address, cfg.username, endpoint
        );

        info!(url, "fetching");

        let resp = reqwest::get(url)
            .await
            .map_err(|_| AppError::NetworkError)
            .unwrap();

        let status = resp.status();

        match status {
            StatusCode::OK => Ok(resp.json().await.map_err(|_| AppError::Other)?),
            StatusCode::NOT_FOUND => Err(AppError::APINotFound),
            _ => Err(AppError::Other),
        }
    }
}

/// get the Hue Bridge ip address
///
pub async fn bridge_ipaddr() -> Result<IpAddr, AppError> {
    const SERVICE_NAME: &'static str = "_hue._tcp.local";

    let responses = mdns::discover::all(SERVICE_NAME, Duration::from_secs(1))
        .map_err(|_| AppError::Other)
        .unwrap()
        .listen();
    pin_mut!(responses);

    match responses.next().await {
        Some(Ok(response)) => {
            let addr = response.records().filter_map(self::to_ip_addr).next();
            if let Some(addr) = addr {
                return Ok(addr);
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

/// read Hue bridge configuration info
///
async fn config_info(ip_address: &IpAddr) -> Result<ConfigInfo, AppError> {
    let url = format!("http://{}/api/0/config", ip_address);

    let resp = reqwest::get(url)
        .await
        .map_err(|_| AppError::NetworkError)
        .unwrap();

    let status = resp.status();
    let text = match status {
        StatusCode::OK => resp.text().await.map_err(|_| AppError::Other),
        StatusCode::NOT_FOUND => Err(AppError::APINotFound),
        _ => Err(AppError::Other),
    }
    .unwrap();

    let config_info: ConfigInfo = serde_json::from_str(&text).unwrap();

    Ok(config_info)
}
