//! PTY error types

/// PTY error types
#[derive(Debug, thiserror::Error)]
pub enum PtyError {
    /// Failed to spawn process
    #[error("Failed to spawn process: {0}")]
    Spawn(String),
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Process not running
    #[error("Process not running")]
    NotRunning,
    /// Send error
    #[error("Failed to send: {0}")]
    Send(String),
}
