///
/// Implement Lights API
///
/// see: https://developers.meethue.com/develop/hue-api/lights-api/#get-new-lights
///
use super::Light;
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
    use super::*;
    use crate::config::AppConfig;
    use crate::config::AuthAppConfig;

    #[tokio::test]
    async fn gets_lights() {
        // create a mock server
        let mock = httpmock::MockServer::start_async().await;

        let test_response = include_str!("test_data/lights.json");

        // set up handlers for specific requests
        let get_lights_mock = mock
            .mock_async(|when, then| {
                when.method("GET").path("/api/auth/lights");
                then.status(200).body(test_response);
            })
            .await;

        // set up the bridge with the mock server's ip + port (socket addr)
        let bridge = Bridge::new_with_config(AppConfig::Auth(AuthAppConfig {
            key: "auth".to_string(),
            ip: mock.address().to_owned(),
            // ip: SocketAddr::V4(SocketAddrV4::new(std::net::Ipv4Addr::new(10, 1, 1, 1), 80)),
        }));

        // make a request
        let lights = bridge.lights().await.unwrap();

        // ensure that the api is called exactly once
        get_lights_mock.assert_hits_async(1).await;
        let l = lights[&1].clone();

        assert_eq!(l.capabilities.certified, true);
        assert_eq!(l.state.on.unwrap(), true);
        assert_eq!(l.state.brightness.unwrap(), 254);

        // println!("{:#?}", lights[&1]);
    }
}
