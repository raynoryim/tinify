use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TinifyError {
    #[error("API key invalid or missing")]
    InvalidApiKey,

    #[error("Monthly quota exceeded")]
    QuotaExceeded,

    #[error("File too large: {size} bytes (max: {max_size} bytes)")]
    FileTooLarge { size: u64, max_size: u64 },

    #[error("Unsupported file format: {format}")]
    UnsupportedFormat { format: String },

    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    #[error("Rate limit exceeded, retry after {retry_after} seconds")]
    RateLimitExceeded { retry_after: u64 },

    #[error("Invalid resize dimensions: width={width:?}, height={height:?}")]
    InvalidDimensions {
        width: Option<u32>,
        height: Option<u32>,
    },

    #[error("Client not initialized. Call TinifyClient::new() or use TinifyClientBuilder")]
    ClientNotInitialized,

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
