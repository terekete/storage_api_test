use std::fmt;
use hyper::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// HTTP error response
    HttpError(StatusCode),
    /// Error during request
    RequestError(String),
    /// Error deserializing response
    DeserializationError(String),
    /// Error serializing request
    SerializationError(String),
    /// Authentication error
    AuthError(String),
    /// Generic error
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HttpError(status) => write!(f, "HTTP error: {}", status),
            Error::RequestError(e) => write!(f, "Request error: {}", e),
            Error::DeserializationError(e) => write!(f, "Failed to deserialize response: {}", e),
            Error::SerializationError(e) => write!(f, "Failed to serialize request: {}", e),
            Error::AuthError(e) => write!(f, "Authentication error: {}", e),
            Error::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::RequestError(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::DeserializationError(err.to_string())
    }
}

impl From<yup_oauth2::Error> for Error {
    fn from(err: yup_oauth2::Error) -> Self {
        Error::AuthError(err.to_string())
    }
}