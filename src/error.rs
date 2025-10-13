use thiserror::Error;

/// Error types for Turbine LLM operations.
///
/// All errors that can occur when using the Turbine LLM library.
#[derive(Error, Debug)]
pub enum TurbineError {
    /// API key environment variable not found
    #[error("API key not found for provider: {0}")]
    ApiKeyNotFound(String),

    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON parsing failed
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    /// API returned an error response
    #[error("API returned error: {0}")]
    ApiError(String),

    /// Response format is invalid or unexpected
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    /// Environment variable error
    #[error("Environment variable error: {0}")]
    EnvError(#[from] std::env::VarError),

    /// Required field is missing
    #[error("Missing required field: {0}")]
    MissingField(String),
}

/// Convenience type alias for Results that may return [`TurbineError`].
pub type Result<T> = std::result::Result<T, TurbineError>;
