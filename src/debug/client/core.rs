//! DAP Client Core
//!
//! Core client structure and basic operations.

use std::collections::HashMap;
use std::process::{Child, ChildStdin};
use std::sync::atomic::AtomicI64;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::super::protocol::*;
use super::types::*;

/// DAP client for communicating with debug adapters
pub struct DapClient {
    pub(crate) config: DapClientConfig,
    pub(crate) process: Option<Child>,
    pub(crate) stdin: Option<ChildStdin>,
    pub(crate) seq: AtomicI64,
    pub(crate) pending_requests: Arc<Mutex<HashMap<i64, PendingRequest>>>,
    pub(crate) event_sender: Option<tokio::sync::mpsc::UnboundedSender<DapEvent>>,
    pub(crate) capabilities: Option<Capabilities>,
    pub(crate) initialized: bool,
}

impl DapClient {
    /// Create a new DAP client
    pub fn new(config: DapClientConfig) -> Self {
        Self {
            config,
            process: None,
            stdin: None,
            seq: AtomicI64::new(1),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            event_sender: None,
            capabilities: None,
            initialized: false,
        }
    }

    /// Get capabilities
    pub fn capabilities(&self) -> Option<&Capabilities> {
        self.capabilities.as_ref()
    }

    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}
