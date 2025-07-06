use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Collision: {0}")]
    Collision(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("Rate limited: {0}")]
    RateLimited(String),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Missing server url")]
    MissingServerUrl,
    #[error("Invalid server url")]
    InvalidServerUrl,
    #[error("Communication error: {0}")]
    Communication(String),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Connection timeout")]
    ConnectionTimeout,
}
