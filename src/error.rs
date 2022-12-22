#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("network error")]
    NetworkError,

    #[error("no Hue bridge found")]
    HueBridgeNotFoundError,

    #[error("no Hue bridge address found")]
    HueBridgeAddressNotFoundError,

    #[error("other error")]
    Other,
}
