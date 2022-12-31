use crate::{error::AppError, hue::Bridge};
use serde::{Deserialize, Serialize};

impl Bridge {
    pub async fn state_for_light(&self, id: &u32) -> Result<LightState, AppError> {
        let url = format!("lights/{}", id);
        let state_response: StateResponse = self.get(&url).await?;

        Ok(state_response.state)
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone, PartialOrd)]
pub struct StateResponse {
    state: LightState,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hue::api::LightAlert;

    #[test]
    fn test_lightalert_into() {
        let m: LightAlert = "lselect".into();
        assert_eq!(m, LightAlert::LSelect);
    }

    #[test]
    fn test_lightalert_from() {
        let m = LightAlert::from("lselect");
        assert_eq!(m, LightAlert::LSelect);
    }

    #[test]
    fn test_serialise_on() {
        let state = LightStateBuilder::default().on(true).build().unwrap();

        let json_text = serde_json::to_string(&state).unwrap();
        let expected = "{\"on\":true}";

        assert_eq!(expected, json_text);
    }

    #[test]
    fn test_xy() {
        let state = LightStateBuilder::default()
            .xy(XY { x: 3.1, y: 4.2 })
            .build()
            .unwrap();

        let json_text = serde_json::to_string(&state).unwrap();
        let expected = "{\"xy\":[3.1,4.2]}";

        assert_eq!(expected, json_text);
    }
}

////////////////////////////////////////////////////////////////////////////////
