//! A Hue bridge

use crate::config;
use crate::error::AppError;
use colored::Colorize;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use tracing::trace;

use self::api::state_for_light::LightState;

pub mod api;

/// A Hue Bridge client providing API commands
///
#[derive(Debug)]
pub struct Bridge {
    pub config: config::AppConfig,
    pub client: reqwest::Client,
    //     pub config_info: ConfigInfo,
}

impl Bridge {
    /// Create a new Hue bridge instance
    pub async fn new() -> Self {
        if !config::is_configured() {
            match config::configure().await {
                Ok(_) => {}
                Err(err) => {
                    println!("{}: {}\n", "error".red().bold(), err.to_string().bold());
                    std::process::exit(1);
                }
            }
        }

        Self::new_with_config(config::get_app_cfg())
    }

    pub fn new_with_config(config: config::AppConfig) -> Self {
        let client = reqwest::Client::builder().build().unwrap();

        Self { config, client }
    }

    /// Gets an endpoint response and deserialises it.
    #[tracing::instrument(skip(self))]
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, AppError> {
        let url = self.get_url(endpoint)?;

        trace!(url, "fetching");

        let resp = self
            .client
            .get(url)
            .timeout(std::time::Duration::from_millis(500))
            .send()
            // we have a From impl from reqwest::Error to AppError
            // so we can use the ? operator to convert the error automatically
            .await?;

        let status = resp.status();

        match status {
            StatusCode::OK => Ok(resp.json().await?),
            StatusCode::NOT_FOUND => Err(AppError::APINotFound),
            _ => Err(AppError::Other),
        }
    }

    /// Puts the given data to the given endpoint
    pub async fn put(&self, endpoint: &str, data: &LightState) -> Result<(), AppError> {
        let url = self.get_url(endpoint)?;
        let body_json = serde_json::to_string(data).unwrap();
        let resp = self.client.put(&url).body(body_json).send().await.unwrap();

        match resp.status() {
            StatusCode::OK => Ok(()),
            StatusCode::NOT_FOUND => Err(AppError::APINotFound),
            _ => Err(AppError::Other),
        }
    }

    fn get_url(&self, endpoint: &str) -> Result<String, AppError> {
        // if we match on the object, the internals are moved into the match statement
        // this allows you to do things like
        // let url = match option {
        //     Some(s) => s,
        //     None => todo!(),
        // }
        // now url 'owns' the data that was in the option variable
        match &self.config {
            config::AppConfig::Uninit => Err(AppError::Other),
            config::AppConfig::Unauth { ip } => Ok(format!("http://{}/api/{}", ip, endpoint)),
            config::AppConfig::Auth(config::AuthAppConfig { ip, key }) => {
                Ok(format!("http://{}/api/{}/{}", ip, key, endpoint))
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
