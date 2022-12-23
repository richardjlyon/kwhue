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
    pub async fn lights(&self) -> Result<JsonMap, AppError> {
        // ALEX how to pass deserialise type into get and deserialise there
        let json_text = self.get("lights").await?;
        let data: JsonMap = serde_json::from_str(&json_text).unwrap();

        Ok(data)
    }
}

/// Hue light schema
///
#[derive(Deserialize, Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
pub struct Light {
    capabilities: Capabilities,
    config: Config,
    pub state: State,
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
pub struct State {
    alert: String,
    #[serde(rename = "bri")]
    brightness: u32,
    #[serde(rename = "colormode")]
    colour_mode: Option<String>,
    #[serde(rename = "ct")]
    colour_temperature: Option<u32>,
    mode: String,
    pub on: bool,
    pub reachable: bool,
}
