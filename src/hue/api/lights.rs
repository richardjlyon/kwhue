///
/// Implement Lights API
///
/// see: https://developers.meethue.com/develop/hue-api/lights-api/#get-new-lights
///
use super::{Light, LightState, LightStateBuilder, XY};
use crate::error::AppError;
use crate::hue::Bridge;
use std::collections::HashMap;

impl Bridge {
    #[tracing::instrument(skip(self))]
    pub async fn lights(&self) -> Result<JsonMap, AppError> {
        tracing::debug!("getting lights");
        let data: JsonMap = self.get("lights").await?;

        Ok(data)
    }
}

type JsonMap = HashMap<u32, Light>;

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::hue::api::LightAlert;

    use super::*;

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
