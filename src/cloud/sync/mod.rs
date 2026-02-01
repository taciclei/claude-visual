//! Conversation Sync Module
//!
//! Handles offline-first synchronization of conversations with the cloud.

mod conflict;
mod manager;
mod persistence;
mod queue;
mod remote;
mod types;

pub use manager::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud::storage::StorageItemType;
    use std::collections::HashMap;

    #[test]
    fn test_sync_state_default() {
        let state = SyncState::default();
        assert!(state.last_sync.is_none());
        assert!(state.id_mapping.is_empty());
        assert_eq!(state.pending_count, 0);
    }

    #[test]
    fn test_sync_config_default() {
        let config = SyncConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.sync_interval_secs, 60);
    }

    #[test]
    fn test_queue_item_creation() {
        let operation = SyncOperation::Create {
            item_type: StorageItemType::Conversation,
            local_id: "local-123".to_string(),
            data: vec![1, 2, 3],
            tags: HashMap::new(),
        };

        assert_eq!(operation.local_id(), "local-123");
    }
}
