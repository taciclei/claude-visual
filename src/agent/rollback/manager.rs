//! Rollback manager core structure

use std::collections::HashMap;

use super::types::*;

/// Rollback manager for tracking and executing rollbacks
pub struct RollbackManager {
    /// Checkpoints (ordered by creation time)
    pub(crate) checkpoints: Vec<RollbackCheckpoint>,
    /// Current (uncommitted) checkpoint
    pub(crate) current: Option<RollbackCheckpoint>,
    /// Maximum number of checkpoints to keep
    pub(crate) max_checkpoints: usize,
    /// Custom rollback handlers
    pub(crate) custom_handlers:
        HashMap<String, Box<dyn Fn(&serde_json::Value) -> Result<(), String> + Send + Sync>>,
}

impl Default for RollbackManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RollbackManager {
    /// Create a new rollback manager
    pub fn new() -> Self {
        Self {
            checkpoints: Vec::new(),
            current: None,
            max_checkpoints: 50,
            custom_handlers: HashMap::new(),
        }
    }

    /// Set maximum checkpoints to keep
    pub fn with_max_checkpoints(mut self, max: usize) -> Self {
        self.max_checkpoints = max;
        self
    }

    /// Register a custom rollback handler
    pub fn register_handler<F>(&mut self, name: impl Into<String>, handler: F)
    where
        F: Fn(&serde_json::Value) -> Result<(), String> + Send + Sync + 'static,
    {
        self.custom_handlers.insert(name.into(), Box::new(handler));
    }

    /// Clear all checkpoints
    pub fn clear(&mut self) {
        self.checkpoints.clear();
        self.current = None;
    }

    /// Get total number of checkpoints
    pub fn checkpoint_count(&self) -> usize {
        self.checkpoints.len()
    }

    /// Get total number of operations across all checkpoints
    pub fn total_operations(&self) -> usize {
        self.checkpoints.iter().map(|c| c.operation_count()).sum()
    }

    /// Check if there's an active checkpoint
    pub fn has_active_checkpoint(&self) -> bool {
        self.current.is_some()
    }

    /// Get current checkpoint operation count
    pub fn current_operations(&self) -> usize {
        self.current
            .as_ref()
            .map(|c| c.operation_count())
            .unwrap_or(0)
    }
}
