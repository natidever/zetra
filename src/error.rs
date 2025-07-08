use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;

#[derive(Debug)]
pub enum Error {
    ServerError,
    InvalidUrl,
    RedisError(redis::RedisError), // Add Redis error variant
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::RedisError(err)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::ServerError => (
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Internal Server Error"
            ).into_response(),
            Error::InvalidUrl => (
                StatusCode::BAD_REQUEST, 
                "Invalid URL"
            ).into_response(),
            Error::RedisError(e) => (
                StatusCode::SERVICE_UNAVAILABLE, 
                format!("Redis Error: {}", e)
            ).into_response(),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;