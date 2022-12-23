///
/// Implement Lights API
///
/// see: https://developers.meethue.com/develop/hue-api/lights-api/#get-new-lights
///
use serde::Deserialize;
use std::collections::HashMap;

use crate::error::AppError;
use crate::hue::bridge::Bridge;

type JsonMap = HashMap<u32, Light>;

impl Bridge {
    #[tracing::instrument(skip(self))]
    pub async fn lights(&self) -> Result<JsonMap, AppError> {
        tracing::debug!("getting lights");
        let data: JsonMap = self.get("lights").await?;

        Ok(data)
    }

    pub async fn get_state(&self, id: &u32) -> Result<(LightState), AppError> {
        let url = format!("lights/{}", id);
        let state_response: StateResponse = self.get(&url).await?;

        Ok(state_response.state)
    }
}

/// Hue light schema
///
#[derive(Deserialize, Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
pub struct Light {
    capabilities: Capabilities,
    config: Config,
    pub state: LightState,
    pub name: String,

    #[serde(rename = "manufacturername")]
    manufacturer_name: String,

    #[serde(rename = "modelid")]
    model_id: String,

    #[serde(rename = "productid")]
    product_id: Option<String>,

    #[serde(rename = "productname")]
    product_name: String,

    #[serde(rename = "type")]
    light_type: String,

    #[serde(rename = "uniqueid")]
    unigue_id: String,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
pub struct Capabilities {
    certified: bool,
    control: Control,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
pub struct Control {
    #[serde(rename = "ct")]
    colour_temperature: Option<Range>,

    #[serde(rename = "maxlumen")]
    max_lumen: u32,

    #[serde(rename = "mindimlevel")]
    min_im_level: u32,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
pub struct Range {
    max: u32,
    min: u32,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
pub struct Config {
    archetype: String,
    direction: String,
    function: String,
    startup: Startup,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
struct Startup {
    configured: bool,
    mode: String,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
pub struct LightState {
    pub alert: String,
    #[serde(rename = "bri")]
    pub brightness: u32,
    #[serde(rename = "colormode")]
    pub colour_mode: Option<String>,
    #[serde(rename = "ct")]
    pub colour_temperature: Option<u32>,
    pub mode: String,
    pub on: bool,
    pub reachable: bool,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
pub struct StateResponse {
    state: LightState,
}
