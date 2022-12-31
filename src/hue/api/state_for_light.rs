use serde::Deserialize;

use crate::{error::AppError, hue::Bridge};

use super::LightState;

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

////////////////////////////////////////////////////////////////////////////////
