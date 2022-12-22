#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("network error")]
    NetworkError,

    #[error("no Hue bridge found")]
    HueBridgeNotFoundError,

    #[error("no Hue bridge address found")]
    HueBridgeAddressNotFoundError,

    #[error("API not found")]
    APINotFound,

    #[error("other error")]
    Other,
}
