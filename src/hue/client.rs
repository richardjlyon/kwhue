use crate::error::AppError;
use crate::hue::bridge_ipaddr;
use reqwest::StatusCode;
use serde::Deserialize;

/// A Hue Bridge client providing API commands
use std::net::IpAddr;

pub struct Bridge {
    pub ip_address: IpAddr,
}

// Hue configuration information
#[derive(Deserialize, Debug)]
pub struct ConfigInfo {
    name: String,
    datastoreversion: String,
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

        Self { ip_address }
    }

    /// read configuration info
    pub async fn config(&self) -> Result<ConfigInfo, AppError> {
        let url = format!("http://{}/api/0/config", self.ip_address);

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
}
