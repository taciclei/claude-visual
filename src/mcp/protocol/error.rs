//! MCP error types

/// MCP Error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McpErrorCode {
    /// Parse error
    ParseError = -32700,
    /// Invalid request
    InvalidRequest = -32600,
    /// Method not found
    MethodNotFound = -32601,
    /// Invalid params
    InvalidParams = -32602,
    /// Internal error
    InternalError = -32603,
}

/// MCP error type
#[derive(Debug, Clone, thiserror::Error)]
pub enum McpError {
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Protocol error: {0}")]
    Protocol(String),
    #[error("Server error: {code} - {message}")]
    Server { code: i32, message: String },
    #[error("Timeout waiting for response")]
    Timeout,
    #[error("Server not initialized")]
    NotInitialized,
    #[error("IO error: {0}")]
    Io(String),
}
