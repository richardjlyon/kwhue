//! Handklkes getting configuration information from a Hue bridge
//!

use super::super::Bridge;
use crate::error::AppError;
use serde::Deserialize;

impl Bridge {
    pub async fn config_info(&self) -> Result<ConfigInfo, AppError> {
        let data: ConfigInfo = self.get("config").await?;

        Ok(data)
    }
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

#[cfg(test)]
mod tests {
    use crate::config::{AppConfig, AuthAppConfig};

    use super::*;

    #[tokio::test]
    async fn test_config() {
        let mock = httpmock::MockServer::start_async().await;
        let test_response = include_str!("test_data/config_info.json");
        let config_info_mock = mock
            .mock_async(|when, then| {
                when.method("GET").path("/api/auth/config");
                then.status(200).body(test_response);
            })
            .await;

        let bridge = Bridge::new_with_config(AppConfig::Auth(AuthAppConfig {
            key: "auth".to_string(),
            ip: mock.address().to_owned(),
        }));

        let config_info = bridge.config_info().await.unwrap();

        // ensure that the api is called exactly once
        config_info_mock.assert_hits_async(1).await;

        assert_eq!(config_info.bridge_id, "001788FFFE491465");
        assert_eq!(config_info.apiversion, "1.55.0");
        assert_eq!(config_info.swversion, "1955082050");
        assert_eq!(config_info.ipaddress, "192.168.1.119");
    }
}
