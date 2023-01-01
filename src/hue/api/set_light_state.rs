//! Implement the `set light state` API call
//! https://developers.meethue.com/develop/hue-api/lights-api/#set-light-state

use super::get_light_state::LightState;
use crate::hue::api::get_light_state::LightStateBuilder;
use crate::{error::AppError, hue::Bridge};
use tokio::time::{sleep, Duration};

impl Bridge {
    pub async fn set_light_state(&self, id: &u32, state: &LightState) -> Result<(), AppError> {
        let url = format!("lights/{id}/state");
        self.put(&url, &state).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_light_state() {
        let bridge = Bridge::new().await;
        let mut state = LightStateBuilder::default()
            .on(true)
            .brightness(254)
            .hue(0)
            .build()
            .unwrap();

        for hue in 0..65535 {
            state.hue = Some(hue);
            bridge.set_light_state(&6, &state).await;
            sleep(Duration::from_millis(100)).await;
        }

        // match result {
        //     Ok(_) => {}
        //     Err(_) => {}
        // };
    }
}
