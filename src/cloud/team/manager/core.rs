//! Core TeamManager operations

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::TeamManager;

impl TeamManager {
    /// Create new team manager
    pub fn new(api_url: impl Into<String>) -> Self {
        Self {
            user_id: Arc::new(RwLock::new(None)),
            teams: Arc::new(RwLock::new(HashMap::new())),
            projects: Arc::new(RwLock::new(HashMap::new())),
            activities: Arc::new(RwLock::new(Vec::new())),
            invitations: Arc::new(RwLock::new(Vec::new())),
            api_url: api_url.into(),
        }
    }

    /// Set current user
    pub async fn set_user(&self, user_id: Option<String>) {
        *self.user_id.write().await = user_id;
    }

    /// Get current user ID
    pub async fn user_id(&self) -> Option<String> {
        self.user_id.read().await.clone()
    }
}
