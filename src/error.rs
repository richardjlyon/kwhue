#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("network error")]
    NetworkError(#[from] reqwest::Error),

    #[error("Hue bridge not found")]
    HueBridgeNotFoundError,

    #[error("Hue bridge ip address not found")]
    HueBridgeAddressNotFoundError,

    #[error("Hue bridge timeout")]
    HueBridgeTimeout,

    #[error("Hue bridge misconfigured")]
    HueBridgeMisconfigured,

    #[error("Hue bridge authorisation key invalid")]
    HueBridgeAuthKeyInvalid,

    #[error("API not found")]
    APINotFound,

    #[error("couldn't deserialize")]
    DeserializeError(#[from] serde_json::Error),

    #[error("other error")]
    Other,
}
