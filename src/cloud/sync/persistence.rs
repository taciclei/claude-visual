use tokio::sync::mpsc;

use super::types::*;
use super::manager::SyncManager;

impl SyncManager {
    /// Persist queue to disk
    pub(super) async fn persist_queue(&self) -> Result<(), SyncError> {
        let queue = self.queue.read().await;
        let items: Vec<_> = queue.iter().cloned().collect();
        let data =
            serde_json::to_string_pretty(&items).map_err(|e| SyncError::Serialization(e.to_string()))?;

        // Ensure directory exists
        if let Some(parent) = self.queue_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| SyncError::Queue(e.to_string()))?;
        }

        tokio::fs::write(&self.queue_path, data)
            .await
            .map_err(|e| SyncError::Queue(e.to_string()))?;

        Ok(())
    }

    /// Persist state to disk
    pub(super) async fn persist_state(&self) -> Result<(), SyncError> {
        let state = self.state.read().await;
        let data =
            serde_json::to_string_pretty(&*state).map_err(|e| SyncError::Serialization(e.to_string()))?;

        // Ensure directory exists
        if let Some(parent) = self.state_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| SyncError::Queue(e.to_string()))?;
        }

        tokio::fs::write(&self.state_path, data)
            .await
            .map_err(|e| SyncError::Queue(e.to_string()))?;

        Ok(())
    }

    /// Subscribe to sync events
    pub fn subscribe(&self) -> mpsc::UnboundedReceiver<SyncEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        // Clone the sender for the subscriber
        // Note: This is a simplified implementation
        // In production, you'd want a proper pub-sub system
        let _ = tx; // Subscriber would hold this
        rx
    }
}
