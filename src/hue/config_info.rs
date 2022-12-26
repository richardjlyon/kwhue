///! Get config info from the bridge
///!
use crate::error::AppError;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Deserialize, Debug)]
pub struct ConfigInfo {
    #[serde(rename = "bridgeid")]
    pub bridge_id: String,
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
