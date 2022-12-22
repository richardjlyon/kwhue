use std::collections::HashMap;
use std::io::Write;

use crate::error::AppError;
use crate::hue::bridge::Bridge;
use reqwest::StatusCode;
use std::{thread, time};
use serde::Deserialize;
///
/// Create a new Hue Bridge user

/// jsom example
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

#[derive(Deserialize, Debug)]
struct UsernameResponse {
    username: String,
}

#[derive(Deserialize, Debug)]
struct Error<T> {
    error: T,
}

#[derive(Deserialize, Debug)]
struct BasicError {
    #[serde(rename = "type")]
    error_type: u32,
    address: String,
    description: String,
}

impl Bridge {
    pub async fn new_user(&self, name: &str) {
        let url = format!("http://{}/api", self.ip_address);
        let client = reqwest::Client::new();

        let mut params = HashMap::new();
        params.insert("devicetype", format!("my_hue_app#iphone {}", &name));
        let one_second = time::Duration::from_secs(1);

        println!("Press link button");

        let username_response = loop {
            let resp = client.post(&url).json(&params).send().await.unwrap();
            let status = resp.status();
            let mut data: Vec<Response<UsernameResponse, BasicError>> = match status {
                StatusCode::OK => resp.json().await.map_err(|_| AppError::Other),
                StatusCode::NOT_FOUND => Err(AppError::APINotFound),
                _ => Err(AppError::Other),
            }
            .unwrap();

            match data.pop().unwrap() {
                Response::Error(e) => {
                    if e.error.error_type == 101 {
                        println!("Link button not pressed");
                    } else {
                        println!("Error: {}", e.error.description);
                    }

                    tokio::time::sleep(one_second).await;
                    print!(".");
                    std::io::stdout().flush().unwrap();
                }
                Response::Success(s) => break s.success
            }
        };

        println!("{:?}", username_response);
    }
}
