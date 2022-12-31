//! A Hue bridge

use crate::config::*;
use crate::error::AppError;
use colored::Colorize;
use futures_util::{pin_mut, stream::StreamExt};
use lights::LightState;
use mdns::{Record, RecordKind};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::Write;
use std::{
    collections::HashMap, net::IpAddr, net::SocketAddr, net::SocketAddrV4, net::SocketAddrV6,
    time::Duration,
};
use tokio::time::timeout;
use tracing::trace;

pub mod api;
pub mod lights;

/// Represents a response from the bridge
///
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

/// Bridge 'error' response
#[derive(Deserialize, Debug)]
struct Error<T> {
    error: T,
}

/// Bridge 'sucess' response
#[derive(Deserialize, Debug)]
struct Success<T> {
    success: T,
}

/// Bridge authorisation key data
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthKeyResponse {
    pub username: String,
}

/// Bridge error data
#[derive(Deserialize, Debug)]
struct BasicError {
    #[serde(rename = "type")]
    error_type: u32,
    description: String,
}

#[derive(Debug)]
pub enum BridgeStatus {
    CONNECTED,
    DISCONNECTED,
}
/// A Hue Bridge client providing API commands
///
#[derive(Debug)]
pub struct Bridge {
    pub config: AppConfig,
    pub client: reqwest::Client,
    //     pub config_info: ConfigInfo,
}

#[derive(Deserialize, Debug)]
pub struct ConfigInfo {
    #[serde(rename = "bridgeid")]
    pub bridge_id: String,
    pub apiversion: String,
    pub swversion: String,
    pub ipaddress: String,
}

impl ConfigInfo {
    pub fn software_version(&self) -> String {
        // 1.55.0 -> 1.55
        let parts: Vec<&str> = self.apiversion.split(".").collect();
        format!("{}.{}.{}", parts[0], parts[1], self.swversion)
    }
}

impl Bridge {
    /// Create a new Hue bridge instance
    pub async fn new() -> Self {
        if !is_configured() {
            match configure().await {
                Ok(_) => {}
                Err(err) => {
                    println!("{}: {}\n", "error".red().bold(), err.to_string().bold());
                    std::process::exit(1);
                }
            }
        }

        Self::new_with_config(get_app_cfg())
    }

    pub fn new_with_config(config: AppConfig) -> Self {
        let client = reqwest::Client::builder().build().unwrap();

        Self { config, client }
    }
}

impl Bridge {
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

    pub async fn config_info(&self) -> Result<ConfigInfo, AppError> {
        let data: ConfigInfo = self.get("config").await?;

        Ok(data)
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
            AppConfig::Uninit => Err(AppError::Other),
            AppConfig::Unauth { ip } => Ok(format!("http://{}/api/{}", ip, endpoint)),
            AppConfig::Auth(AuthAppConfig { ip, key }) => {
                Ok(format!("http://{}/api/{}/{}", ip, key, endpoint))
            }
        }
    }
}

/// Return true if config file contains an ip address and auth key.
fn is_configured() -> bool {
    get_app_cfg().is_configured()
}

/// Gets the IP address, creates an auth_key, and saves both to the config file.
async fn configure() -> Result<AuthAppConfig, AppError> {
    let ipaddr = get_bridge_socket().await?;
    let auth_key = create_new_auth_key(ipaddr).await?;

    let cfg = AuthAppConfig {
        ip: ipaddr,
        key: auth_key,
    };

    store_app_cfg(&AppConfig::Auth(cfg.clone()));

    Ok(cfg)
}
/// Gets the Hue Bridge ip address.
pub async fn get_bridge_socket() -> Result<SocketAddr, AppError> {
    const SERVICE_NAME: &str = "_hue._tcp.local";

    let responses = mdns::discover::all(SERVICE_NAME, Duration::from_secs(1))
        .map_err(|_| AppError::Other)?
        .listen();

    pin_mut!(responses);

    // if the hub is disconnected, it will timeout and block the app
    let response = match timeout(Duration::from_secs(5), responses.next()).await {
        Ok(r) => Ok(r),
        Err(_) => Err(AppError::HueBridgeTimeout),
    };

    match response {
        Ok(r) => match r {
            Some(Ok(response)) => {
                let addr = response.records().filter_map(self::to_socket_addr).next();
                if let Some(addr) = addr {
                    Ok(addr)
                } else {
                    Err(AppError::HueBridgeAddressNotFoundError)
                }
            }
            Some(Err(_)) => todo!(),
            None => Err(AppError::Other),
        },
        Err(err) => Err(err),
    }
}

/// Creates a hub authorisation key.
///
/// See [Hue Configuration API](https://developers.meethue.com/develop/hue-api/7-configuration-api/)
pub async fn create_new_auth_key(socket_addr: SocketAddr) -> Result<String, AppError> {
    let url = format!("http://{}/api", socket_addr);
    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("devicetype", "kwhue_app rust_app");

    print!("Press link button");
    std::io::stdout().flush().unwrap();

    let response = loop {
        let resp = client.post(&url).json(&params).send().await.unwrap();
        let status = resp.status();

        let mut data: Vec<Response<AuthKeyResponse, BasicError>> = match status {
            StatusCode::OK => resp.json().await.map_err(|err| {
                println!("error: {}", err);
                AppError::HueBridgeAuthKeyInvalid
            }),
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
                tokio::time::sleep(Duration::from_secs(1)).await;
                print!(".");
                std::io::stdout().flush().unwrap();
            }
            Response::Success(s) => break s.success,
        }
    };

    Ok(response.username)
}

///
pub async fn bridge_status() -> BridgeStatus {
    let result = get_bridge_socket().await;
    match result {
        Ok(_) => BridgeStatus::CONNECTED,
        Err(_) => BridgeStatus::DISCONNECTED,
    }
}

fn to_socket_addr(record: &Record) -> Option<SocketAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(SocketAddr::new(addr.into(), 80)),
        RecordKind::AAAA(addr) => Some(SocketAddr::new(addr.into(), 80)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn gets_lights() {
        // create a mock server
        let mock = httpmock::MockServer::start_async().await;

        // set up handlers for specific requests
        let get_lights = mock
            .mock_async(|when, then| {
                when.method("GET").path("/api/auth/lights");
                then.status(200).body(
                    r#"{
                        "1": {
                            "state": {
                                "on": true,
                                "bri": 254,
                                "ct": 366,
                                "alert": "select",
                                "colormode": "ct",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-08T14:24:17"
                            },
                            "type": "Color temperature light",
                            "name": "Bedroom Blue Shade",
                            "modelid": "LTW010",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue ambiance lamp",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 1000,
                                    "maxlumen": 806,
                                    "ct": {
                                        "min": 153,
                                        "max": 454
                                    }
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "tableshade",
                                "function": "functional",
                                "direction": "omnidirectional",
                                "startup": {
                                    "mode": "safety",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:04:3f:19:c5-0b",
                            "swversion": "1.101.2",
                            "swconfigid": "683EA546",
                            "productid": "Philips-LTW010-1-A19CTv2"
                        },
                        "2": {
                            "state": {
                                "on": true,
                                "bri": 254,
                                "ct": 366,
                                "alert": "select",
                                "colormode": "ct",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-08T14:24:39"
                            },
                            "type": "Color temperature light",
                            "name": "Office Desk Left",
                            "modelid": "LTW001",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue ambiance lamp",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 1000,
                                    "maxlumen": 806,
                                    "ct": {
                                        "min": 153,
                                        "max": 454
                                    }
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "flexiblelamp",
                                "function": "functional",
                                "direction": "omnidirectional",
                                "startup": {
                                    "mode": "safety",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:02:36:fd:03-0b",
                            "swversion": "67.101.2"
                        },
                        "4": {
                            "state": {
                                "on": false,
                                "bri": 65,
                                "ct": 443,
                                "alert": "none",
                                "colormode": "ct",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-08T14:30:28"
                            },
                            "type": "Color temperature light",
                            "name": "Lounge Piano",
                            "modelid": "LTW001",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue ambiance lamp",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 1000,
                                    "maxlumen": 806,
                                    "ct": {
                                        "min": 153,
                                        "max": 454
                                    }
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "pendantround",
                                "function": "functional",
                                "direction": "omnidirectional",
                                "startup": {
                                    "mode": "lastonstate",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:02:de:7b:51-0b",
                            "swversion": "67.101.2"
                        },
                        "5": {
                            "state": {
                                "on": false,
                                "bri": 204,
                                "hue": 12868,
                                "sat": 52,
                                "effect": "none",
                                "xy": [
                                    0.4578,
                                    0.4121
                                ],
                                "alert": "none",
                                "colormode": "xy",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2021-10-19T13:26:58"
                            },
                            "type": "Color light",
                            "name": "Lounge Wall Bloom",
                            "modelid": "LLC011",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue bloom",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 10000,
                                    "maxlumen": 120,
                                    "colorgamuttype": "A",
                                    "colorgamut": [
                                        [
                                            0.704,
                                            0.296
                                        ],
                                        [
                                            0.2151,
                                            0.7106
                                        ],
                                        [
                                            0.138,
                                            0.08
                                        ]
                                    ]
                                },
                                "streaming": {
                                    "renderer": true,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "huebloom",
                                "function": "decorative",
                                "direction": "upwards",
                                "startup": {
                                    "mode": "safety",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:03:31:af:8a-0b",
                            "swversion": "67.93.11"
                        },
                        "6": {
                            "state": {
                                "on": true,
                                "bri": 100,
                                "hue": 14814,
                                "sat": 252,
                                "effect": "none",
                                "xy": [
                                    0.5199,
                                    0.435
                                ],
                                "ct": 484,
                                "alert": "select",
                                "colormode": "xy",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-09T14:13:16"
                            },
                            "type": "Extended color light",
                            "name": "TV Wicker Lamp",
                            "modelid": "LCT001",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue color lamp",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 5000,
                                    "maxlumen": 600,
                                    "colorgamuttype": "B",
                                    "colorgamut": [
                                        [
                                            0.675,
                                            0.322
                                        ],
                                        [
                                            0.409,
                                            0.518
                                        ],
                                        [
                                            0.167,
                                            0.04
                                        ]
                                    ],
                                    "ct": {
                                        "min": 153,
                                        "max": 500
                                    }
                                },
                                "streaming": {
                                    "renderer": true,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "tableshade",
                                "function": "mixed",
                                "direction": "omnidirectional",
                                "startup": {
                                    "mode": "custom",
                                    "configured": true,
                                    "customsettings": {
                                        "bri": 254,
                                        "xy": [
                                            0.4806,
                                            0.4667
                                        ]
                                    }
                                }
                            },
                            "uniqueid": "00:17:88:01:00:b5:3e:34-0b",
                            "swversion": "67.101.2"
                        },
                        "7": {
                            "state": {
                                "on": true,
                                "bri": 254,
                                "ct": 366,
                                "alert": "select",
                                "colormode": "ct",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-08T14:29:53"
                            },
                            "type": "Color temperature light",
                            "name": "Office Desk Right",
                            "modelid": "LTW001",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue ambiance lamp",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 1000,
                                    "maxlumen": 806,
                                    "ct": {
                                        "min": 153,
                                        "max": 454
                                    }
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "flexiblelamp",
                                "function": "functional",
                                "direction": "omnidirectional",
                                "startup": {
                                    "mode": "safety",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:02:36:fb:c6-0b",
                            "swversion": "67.101.2"
                        },
                        "8": {
                            "state": {
                                "on": true,
                                "bri": 254,
                                "hue": 15380,
                                "sat": 254,
                                "effect": "none",
                                "xy": [
                                    0.5211,
                                    0.4614
                                ],
                                "alert": "none",
                                "colormode": "xy",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2021-10-28T13:44:38"
                            },
                            "type": "Color light",
                            "name": "TV Lightstrip",
                            "modelid": "LST001",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue lightstrip",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 10000,
                                    "maxlumen": 120,
                                    "colorgamuttype": "A",
                                    "colorgamut": [
                                        [
                                            0.704,
                                            0.296
                                        ],
                                        [
                                            0.2151,
                                            0.7106
                                        ],
                                        [
                                            0.138,
                                            0.08
                                        ]
                                    ]
                                },
                                "streaming": {
                                    "renderer": true,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "huelightstrip",
                                "function": "mixed",
                                "direction": "omnidirectional",
                                "startup": {
                                    "mode": "safety",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:00:cf:9f:c6-0b",
                            "swversion": "67.93.11"
                        },
                        "9": {
                            "state": {
                                "on": true,
                                "bri": 100,
                                "alert": "none",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-08T14:23:15"
                            },
                            "type": "Dimmable light",
                            "name": "TV Alcove 1",
                            "modelid": "LWG004",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue white spot",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 2000,
                                    "maxlumen": 350
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "ceilinground",
                                "function": "functional",
                                "direction": "downwards",
                                "startup": {
                                    "mode": "safety",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:09:db:8e:d7-0b",
                            "swversion": "1.101.7",
                            "swconfigid": "BA9F23D6",
                            "productid": "Philips-LWG004-3-GU10DLv2"
                        },
                        "10": {
                            "state": {
                                "on": true,
                                "bri": 100,
                                "alert": "none",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-08T14:23:10"
                            },
                            "type": "Dimmable light",
                            "name": "TV Alcove 2",
                            "modelid": "LWG004",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue white spot",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 2000,
                                    "maxlumen": 350
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "ceilinground",
                                "function": "functional",
                                "direction": "downwards",
                                "startup": {
                                    "mode": "safety",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:09:e3:94:72-0b",
                            "swversion": "1.101.7",
                            "swconfigid": "BA9F23D6",
                            "productid": "Philips-LWG004-3-GU10DLv2"
                        },
                        "11": {
                            "state": {
                                "on": true,
                                "bri": 100,
                                "alert": "none",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-08T14:28:15"
                            },
                            "type": "Dimmable light",
                            "name": "TV Alcove 3",
                            "modelid": "LWG004",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue white spot",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 2000,
                                    "maxlumen": 350
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "ceilinground",
                                "function": "functional",
                                "direction": "downwards",
                                "startup": {
                                    "mode": "safety",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:09:e3:91:ce-0b",
                            "swversion": "1.101.7",
                            "swconfigid": "BA9F23D6",
                            "productid": "Philips-LWG004-3-GU10DLv2"
                        },
                        "12": {
                            "state": {
                                "on": false,
                                "bri": 187,
                                "ct": 363,
                                "alert": "select",
                                "colormode": "ct",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-08T14:29:50"
                            },
                            "type": "Color temperature light",
                            "name": "Lobby Ceiling",
                            "modelid": "LTW013",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue ambiance spot",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 1000,
                                    "maxlumen": 250,
                                    "ct": {
                                        "min": 153,
                                        "max": 454
                                    }
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "recessedceiling",
                                "function": "functional",
                                "direction": "downwards",
                                "startup": {
                                    "mode": "safety",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:06:bd:d2:d1-0b",
                            "swversion": "1.101.2",
                            "swconfigid": "116B9B72",
                            "productid": "Philips-LTW013-1-GU10CTv1"
                        },
                        "14": {
                            "state": {
                                "on": false,
                                "bri": 1,
                                "alert": "select",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-08T14:23:26"
                            },
                            "type": "Dimmable light",
                            "name": "Hall Desk Lamp",
                            "modelid": "LWO001",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue filament bulb",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 2000,
                                    "maxlumen": 550
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "vintagebulb",
                                "function": "decorative",
                                "direction": "omnidirectional",
                                "startup": {
                                    "mode": "safety",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:09:03:80:1d-0b",
                            "swversion": "1.101.2",
                            "swconfigid": "9732F7F5",
                            "productid": "Philips-LWO001-1-G93CFDLv1"
                        },
                        "15": {
                            "state": {
                                "on": true,
                                "bri": 41,
                                "ct": 451,
                                "alert": "select",
                                "colormode": "ct",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-17T14:22:46"
                            },
                            "type": "Color temperature light",
                            "name": "Hall lobby",
                            "modelid": "929003045001_03",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue Milliskin spot",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 200,
                                    "maxlumen": 350,
                                    "ct": {
                                        "min": 153,
                                        "max": 454
                                    }
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "recessedceiling",
                                "function": "functional",
                                "direction": "downwards",
                                "startup": {
                                    "mode": "lastonstate",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:0c:96:16:74-0b",
                            "swversion": "1.101.7",
                            "swconfigid": "87D6EF03",
                            "productid": "Philips-LTG002-3-GU10CTv2"
                        },
                        "16": {
                            "state": {
                                "on": true,
                                "bri": 41,
                                "ct": 451,
                                "alert": "select",
                                "colormode": "ct",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-17T14:22:39"
                            },
                            "type": "Color temperature light",
                            "name": "Hall 1",
                            "modelid": "929003045001_01",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue Milliskin spot",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 200,
                                    "maxlumen": 350,
                                    "ct": {
                                        "min": 153,
                                        "max": 454
                                    }
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "recessedceiling",
                                "function": "functional",
                                "direction": "downwards",
                                "startup": {
                                    "mode": "lastonstate",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:0c:9f:40:81-0b",
                            "swversion": "1.101.7",
                            "swconfigid": "87D6EF03",
                            "productid": "Philips-LTG002-3-GU10CTv2"
                        },
                        "17": {
                            "state": {
                                "on": true,
                                "bri": 41,
                                "ct": 451,
                                "alert": "select",
                                "colormode": "ct",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-17T14:22:39"
                            },
                            "type": "Color temperature light",
                            "name": "Hall 2",
                            "modelid": "929003045001_02",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue Milliskin spot",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 200,
                                    "maxlumen": 350,
                                    "ct": {
                                        "min": 153,
                                        "max": 454
                                    }
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "recessedceiling",
                                "function": "functional",
                                "direction": "downwards",
                                "startup": {
                                    "mode": "lastonstate",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:0c:9f:2e:aa-0b",
                            "swversion": "1.101.7",
                            "swconfigid": "87D6EF03",
                            "productid": "Philips-LTG002-3-GU10CTv2"
                        },
                        "18": {
                            "state": {
                                "on": true,
                                "bri": 41,
                                "ct": 451,
                                "alert": "select",
                                "colormode": "ct",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-16T14:58:04"
                            },
                            "type": "Color temperature light",
                            "name": "Hall 5",
                            "modelid": "929003045001_03",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue Milliskin spot",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 200,
                                    "maxlumen": 350,
                                    "ct": {
                                        "min": 153,
                                        "max": 454
                                    }
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "recessedceiling",
                                "function": "functional",
                                "direction": "downwards",
                                "startup": {
                                    "mode": "lastonstate",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:0c:96:09:98-0b",
                            "swversion": "1.101.7",
                            "swconfigid": "87D6EF03",
                            "productid": "Philips-LTG002-3-GU10CTv2"
                        },
                        "19": {
                            "state": {
                                "on": true,
                                "bri": 41,
                                "ct": 451,
                                "alert": "select",
                                "colormode": "ct",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-16T14:58:01"
                            },
                            "type": "Color temperature light",
                            "name": "Hall 3",
                            "modelid": "929003045001_01",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue Milliskin spot",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 200,
                                    "maxlumen": 350,
                                    "ct": {
                                        "min": 153,
                                        "max": 454
                                    }
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "recessedceiling",
                                "function": "functional",
                                "direction": "downwards",
                                "startup": {
                                    "mode": "lastonstate",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:0c:9e:72:4d-0b",
                            "swversion": "1.101.7",
                            "swconfigid": "87D6EF03",
                            "productid": "Philips-LTG002-3-GU10CTv2"
                        },
                        "20": {
                            "state": {
                                "on": true,
                                "bri": 41,
                                "ct": 451,
                                "alert": "select",
                                "colormode": "ct",
                                "mode": "homeautomation",
                                "reachable": true
                            },
                            "swupdate": {
                                "state": "noupdates",
                                "lastinstall": "2022-12-16T15:02:58"
                            },
                            "type": "Color temperature light",
                            "name": "Hall 4",
                            "modelid": "929003045001_02",
                            "manufacturername": "Signify Netherlands B.V.",
                            "productname": "Hue Milliskin spot",
                            "capabilities": {
                                "certified": true,
                                "control": {
                                    "mindimlevel": 200,
                                    "maxlumen": 350,
                                    "ct": {
                                        "min": 153,
                                        "max": 454
                                    }
                                },
                                "streaming": {
                                    "renderer": false,
                                    "proxy": false
                                }
                            },
                            "config": {
                                "archetype": "recessedceiling",
                                "function": "functional",
                                "direction": "downwards",
                                "startup": {
                                    "mode": "lastonstate",
                                    "configured": true
                                }
                            },
                            "uniqueid": "00:17:88:01:0c:96:0e:eb-0b",
                            "swversion": "1.101.7",
                            "swconfigid": "87D6EF03",
                            "productid": "Philips-LTG002-3-GU10CTv2"
                        }
                    }"#,
                );
            })
            .await;

        // set up the bridge with the mock server's ip + port (socket addr)
        let bridge = Bridge::new_with_config(AppConfig::Auth(AuthAppConfig {
            key: "auth".to_string(),
            ip: mock.address().to_owned(),
            // ip: SocketAddr::V4(SocketAddrV4::new(std::net::Ipv4Addr::new(10, 1, 1, 1), 80)),
        }));

        // make a request
        let lights = bridge.get_lights().await.unwrap();

        // ensure that the api is called exactly once
        get_lights.assert_hits_async(1).await;

        let times_requested = get_lights.hits();
        println!("this endpoint was hit {} times", times_requested);
    }
}
