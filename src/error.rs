use thiserror::Error;

#[derive(Error, Debug)]
pub enum TinifyError {
    #[error("Account error: {message}")]
    AccountError {
        message: String,
        error_type: Option<String>,
        status: Option<u16>,
    },

    #[error("Client error: {message}")]
    ClientError {
        message: String,
        error_type: Option<String>,
        status: Option<u16>,
    },

    #[error("Server error: {message}")]
    ServerError {
        message: String,
        error_type: Option<String>,
        status: Option<u16>,
    },

    #[error("Connection error: {0}")]
    ConnectionError(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Unknown error: {message}")]
    UnknownError { message: String },
}

pub type Result<T> = std::result::Result<T, TinifyError>;
