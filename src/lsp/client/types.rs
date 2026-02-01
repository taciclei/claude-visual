//! LSP Client types and data structures

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::oneshot;

use crate::lsp::protocol::*;

/// LSP client configuration
#[derive(Debug, Clone)]
pub struct LspClientConfig {
    /// Server command
    pub command: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Working directory
    pub cwd: Option<PathBuf>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Root URI for the workspace
    pub root_uri: Option<String>,
}

impl LspClientConfig {
    /// Create config for rust-analyzer
    pub fn rust_analyzer(root: PathBuf) -> Self {
        Self {
            command: "rust-analyzer".to_string(),
            args: vec![],
            cwd: Some(root.clone()),
            env: HashMap::new(),
            root_uri: Some(format!("file://{}", root.display())),
        }
    }

    /// Create config for TypeScript language server
    pub fn typescript(root: PathBuf) -> Self {
        Self {
            command: "typescript-language-server".to_string(),
            args: vec!["--stdio".to_string()],
            cwd: Some(root.clone()),
            env: HashMap::new(),
            root_uri: Some(format!("file://{}", root.display())),
        }
    }

    /// Create config for Python language server (pyright)
    pub fn pyright(root: PathBuf) -> Self {
        Self {
            command: "pyright-langserver".to_string(),
            args: vec!["--stdio".to_string()],
            cwd: Some(root.clone()),
            env: HashMap::new(),
            root_uri: Some(format!("file://{}", root.display())),
        }
    }
}

/// JSON-RPC request
#[derive(Debug, Serialize)]
pub(crate) struct JsonRpcRequest {
    pub(crate) jsonrpc: &'static str,
    pub(crate) id: u64,
    pub(crate) method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) params: Option<Value>,
}

/// JSON-RPC notification
#[derive(Debug, Serialize)]
pub(crate) struct JsonRpcNotification {
    pub(crate) jsonrpc: &'static str,
    pub(crate) method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) params: Option<Value>,
}

/// JSON-RPC response
#[derive(Debug, Deserialize)]
pub(crate) struct JsonRpcResponse {
    #[allow(dead_code)]
    pub(crate) jsonrpc: String,
    pub(crate) id: Option<u64>,
    pub(crate) result: Option<Value>,
    pub(crate) error: Option<JsonRpcError>,
}

/// JSON-RPC error
#[derive(Debug, Deserialize)]
pub(crate) struct JsonRpcError {
    pub(crate) code: i32,
    pub(crate) message: String,
    #[allow(dead_code)]
    pub(crate) data: Option<Value>,
}

/// LSP event from server
#[derive(Debug, Clone)]
pub enum LspEvent {
    /// Server initialized
    Initialized(ServerCapabilities),
    /// Diagnostics published
    Diagnostics(String, Vec<Diagnostic>),
    /// Server sent a log message
    LogMessage(String),
    /// Server error
    Error(String),
    /// Server exited
    Exited(Option<i32>),
}

/// Pending request
pub(crate) struct PendingRequest {
    pub(crate) sender: oneshot::Sender<Result<Value, String>>,
}
