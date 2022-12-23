///
/// Get the lights
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::error::AppError;
use crate::get_user_cfg;
use crate::hue::bridge::Bridge;
use reqwest::StatusCode;

type JsonMap = HashMap<u32, Light>;

impl Bridge {
    pub async fn lights(&self) -> Result<JsonMap, AppError>{
        let cfg = get_user_cfg();
        let url = format!("http://{}/api/{}/lights", self.ip_address, cfg.username);

        let resp = reqwest::get(url)
            .await
            .map_err(|_| AppError::NetworkError)
            .unwrap();

        let status = resp.status();

        let data: JsonMap = match status {
            StatusCode::OK => resp.json().await.map_err(|_| AppError::Other),
            StatusCode::NOT_FOUND => Err(AppError::APINotFound),
            _ => Err(AppError::Other),
        }
        .unwrap();
        
        Ok(data)
    }
}

/// Hue light schema
///
#[derive(Deserialize, Debug)]
pub struct Light {
    capabilities: Capabilities,
    config: Config,
    state: State,
    name: String,

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

#[derive(Deserialize, Debug)]
pub struct Capabilities {
    certified: bool,
    control: Control,
}

#[derive(Deserialize, Debug)]
pub struct Control {
    #[serde(rename = "ct")]
    colour_temperature: Option<Range>,

    #[serde(rename = "maxlumen")]
    max_lumen: u32,

    #[serde(rename = "mindimlevel")]
    min_im_level: u32,
}

#[derive(Deserialize, Debug)]
pub struct Range {
    max: u32,
    min: u32,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    archetype: String,
    direction: String,
    function: String,
    startup: Startup,
}

#[derive(Deserialize, Debug)]
struct Startup {
    configured: bool,
    mode: String,
}

#[derive(Deserialize, Debug)]
pub struct State {
    alert: String,
    #[serde(rename = "bri")]
    brightness: u32,
    #[serde(rename = "colormode")]
    colour_mode: Option<String>,
    #[serde(rename = "ct")]
    colour_temperature: Option<u32>,
    mode: String,
    on: bool,
    reachable: bool,
}
