use serde::{Deserialize, Serialize};

use self::state_for_light::LightState;

pub mod config_info;
pub mod lights;
pub mod state_for_light;

#[derive(Deserialize, Debug, PartialEq, Clone, PartialOrd)]
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
    unique_id: String,
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

#[derive(Deserialize, Debug, PartialEq, Hash, Clone, PartialOrd)]
pub struct Config {
    // archetype: String,
    // direction: String,
    // function: String,
    // startup: Startup,
}

#[derive(Debug, PartialEq)]
pub enum LightAlert {
    Select,
    LSelect,
    None,
    Uninitialised,
}

impl From<&str> for LightAlert {
    fn from(s: &str) -> LightAlert {
        match s {
            "select" => LightAlert::Select,
            "lselect" => LightAlert::LSelect,
            "none" => LightAlert::None,
            _ => LightAlert::Uninitialised,
        }
    }
}
