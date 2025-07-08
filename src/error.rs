use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;

pub type Result<T>= core::result::Result<T, Error>;
pub enum Error {
    ServerError,
    InvalidUrl,
}

impl IntoResponse for Error {
    fn into_response(self)-> Response {
       

        (StatusCode::INTERNAL_SERVER_ERROR, "ClientError").into_response()


    }
}