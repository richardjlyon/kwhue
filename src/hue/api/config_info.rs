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
