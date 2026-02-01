//! DAP Client Types
//!
//! Error types, configuration, and supporting structures.

use std::collections::HashMap;
use tokio::sync::oneshot;

use super::super::protocol::*;

/// DAP client error
#[derive(Debug, Clone)]
pub enum DapClientError {
    /// Failed to spawn adapter
    SpawnError(String),
    /// Communication error
    IoError(String),
    /// Protocol error
    ProtocolError(String),
    /// Request failed
    RequestFailed { command: String, message: String },
    /// Timeout
    Timeout,
    /// Not initialized
    NotInitialized,
    /// Already running
    AlreadyRunning,
}

impl std::fmt::Display for DapClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DapClientError::SpawnError(msg) => write!(f, "Failed to spawn adapter: {}", msg),
            DapClientError::IoError(msg) => write!(f, "IO error: {}", msg),
            DapClientError::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
            DapClientError::RequestFailed { command, message } => {
                write!(f, "Request '{}' failed: {}", command, message)
            }
            DapClientError::Timeout => write!(f, "Request timed out"),
            DapClientError::NotInitialized => write!(f, "Client not initialized"),
            DapClientError::AlreadyRunning => write!(f, "Debug session already running"),
        }
    }
}

impl std::error::Error for DapClientError {}

/// DAP client configuration
#[derive(Debug, Clone)]
pub struct DapClientConfig {
    /// Adapter command to run
    pub command: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Working directory
    pub cwd: Option<String>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Request timeout in milliseconds
    pub timeout_ms: u64,
}

impl Default for DapClientConfig {
    fn default() -> Self {
        Self {
            command: String::new(),
            args: Vec::new(),
            cwd: None,
            env: HashMap::new(),
            timeout_ms: 10000,
        }
    }
}

impl DapClientConfig {
    /// Create config for rust-analyzer debugger
    pub fn rust_analyzer() -> Self {
        Self {
            command: "rust-analyzer".to_string(),
            args: vec!["debug".to_string()],
            ..Default::default()
        }
    }

    /// Create config for CodeLLDB
    pub fn codelldb(extension_path: &str) -> Self {
        Self {
            command: format!("{}/adapter/codelldb", extension_path),
            args: vec!["--port".to_string(), "0".to_string()],
            ..Default::default()
        }
    }

    /// Create config for Python debugger (debugpy)
    pub fn debugpy() -> Self {
        Self {
            command: "python".to_string(),
            args: vec![
                "-m".to_string(),
                "debugpy.adapter".to_string(),
            ],
            ..Default::default()
        }
    }

    /// Create config for Node.js debugger
    pub fn node_debug() -> Self {
        Self {
            command: "node".to_string(),
            args: vec![
                "--inspect-brk".to_string(),
            ],
            ..Default::default()
        }
    }

    /// Create config for Go debugger (delve)
    pub fn delve() -> Self {
        Self {
            command: "dlv".to_string(),
            args: vec!["dap".to_string()],
            ..Default::default()
        }
    }
}

/// Pending request tracker
pub(crate) struct PendingRequest {
    pub(crate) sender: oneshot::Sender<Result<DapResponse, DapClientError>>,
}
