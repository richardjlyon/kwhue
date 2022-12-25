///
/// Create a new Hue Bridge user
///
/// A user is created with a name. After creation, the user has to press the
/// botton on the bridge to confirm they are physically present. The bridge
/// returns a code, which is stored in a config file.
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::time;

use crate::error::AppError;
use crate::hue::bridge::Bridge;

/// json example
///
/// { "error": {} } // Error<T>
/// { "success": {} } // Success<T>
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Response<T, E> {
    Error(Error<E>),
    Success(Success<T>),
}

#[derive(Deserialize, Debug)]
struct Success<T> {
    success: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthKeyResponse {
    pub auth_key: String,
}

#[derive(Deserialize, Debug)]
struct Error<T> {
    error: T,
}

// TODO convert error_type to Hue Error messages
// see: https://developers.meethue.com/develop/hue-api/error-messages/
#[derive(Deserialize, Debug)]
struct BasicError {
    #[serde(rename = "type")]
    error_type: u32,
    description: String,
}

/// Create a new bridge auth key
///
impl Bridge {
    pub async fn create_new_auth_key(&self) -> String {
        let url = format!("http://{}/api", self.ip_address);
        let client = reqwest::Client::new();

        let mut params = HashMap::new();
        params.insert("devicetype", "kwhue_app rust_app");

        let one_second = time::Duration::from_secs(1);

        print!("Press link button");
        std::io::stdout().flush().unwrap();

        let response = loop {
            let resp = client.post(&url).json(&params).send().await.unwrap();
            let status = resp.status();
            let mut data: Vec<Response<AuthKeyResponse, BasicError>> = match status {
                StatusCode::OK => resp.json().await.map_err(|_| AppError::Other),
                StatusCode::NOT_FOUND => Err(AppError::APINotFound),
                _ => Err(AppError::Other),
            }
            .unwrap();

            // the hub returns an error until the link button is pressed
            match data.pop().unwrap() {
                Response::Error(e) => {
                    if e.error.error_type != 101 {
                        println!("Error: {}", e.error.description);
                    }
                    tokio::time::sleep(one_second).await;
                    print!(".");
                    std::io::stdout().flush().unwrap();
                }
                Response::Success(s) => break s.success,
            }
        };

        response.auth_key
    }
}
