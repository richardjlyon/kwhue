use serde::{Deserialize, Serialize};

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

#[derive(
    Default, Serialize, Deserialize, Debug, PartialEq, Clone, PartialOrd, derive_builder::Builder,
)]
#[builder(setter(strip_option))]
#[builder(build_fn(validate = "Self::validate"))]
pub struct LightState {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub on: Option<bool>,

    #[serde(rename = "bri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub brightness: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub hue: Option<u32>,

    #[serde(rename = "sat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub saturation: Option<u8>,

    #[serde(rename = "colormode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub colour_mode: Option<String>,

    // pub xy: Option<String>,
    #[serde(rename = "ct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub colour_temperature: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub xy: Option<XY>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub reachable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub alert: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct XY {
    x: f32,
    y: f32,
}

impl<'a> Deserialize<'a> for XY {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let xy: [f32; 2] = Deserialize::deserialize(deserializer)?;
        Ok(XY { x: xy[0], y: xy[1] })
    }
}

impl Serialize for XY {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let xy = [self.x, self.y];
        Serialize::serialize(&xy, serializer)
    }
}

impl LightStateBuilder {
    fn validate(&self) -> Result<(), String> {
        // if self.on.is_none() && self.brightness.is_none() {
        //     return Err("Either on or brightness must be set".to_string());
        // }

        Ok(())
    }
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
