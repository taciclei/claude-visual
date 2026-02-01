//! AI provider error types

use thiserror::Error;

/// AI provider error types
#[derive(Debug, Error)]
pub enum AIError {
    /// Network error
    #[error("Network error: {0}")]
    Network(String),
    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),
    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimit,
    /// Invalid request
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    /// Model not available
    #[error("Model not available: {0}")]
    ModelNotAvailable(String),
    /// Context too long
    #[error("Context too long: {0} tokens exceeds {1} limit")]
    ContextTooLong(usize, usize),
    /// Provider error
    #[error("Provider error: {0}")]
    Provider(String),
    /// Timeout
    #[error("Request timeout")]
    Timeout,
    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}
