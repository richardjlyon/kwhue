use crate::error::AppError;
use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Record, RecordKind};
use reqwest::StatusCode;
use serde::Deserialize;
use std::{net::IpAddr, time::Duration};

/// A Hue Bridge client providing API commands

pub struct Bridge {
    pub ip_address: IpAddr,
    pub config_info: ConfigInfo,
}

// Hue configuration information
#[derive(Deserialize, Debug)]
pub struct ConfigInfo {
    name: String,
    #[serde(rename = "datastoreversion")]
    datastore_version: String,
    swversion: String,
    apiversion: String,
    mac: String,
    bridgeid: String,
    factorynew: bool,
    modelid: String,
}

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

/// get the Hue Bridge ip address
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
async fn config_info(ip_address: &IpAddr) -> Result<ConfigInfo, AppError> {
    let url = format!("http://{}/api/0/config", ip_address);

    println!("url: {}", url);

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
