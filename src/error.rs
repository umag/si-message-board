use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    MessageDeleteFailIdNotFound { id: u64 },
    MessageUpdateFailIdNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - Error: {self:?}", "INTO_RESPONSE");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::LoginFail => write!(f, "Login failed"),
            Error::MessageDeleteFailIdNotFound { id } => {
                write!(f, "Message delete failed, id not found: {}", id)
            },
            Error::MessageUpdateFailIdNotFound { id } => {
                write!(f, "Message update failed, id not found: {}", id)
            },
        }
        
    }
}

impl std::error::Error for Error {}
