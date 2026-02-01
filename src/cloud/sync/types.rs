use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

use crate::cloud::storage::{StorageError, StorageItemType};

/// Sync error types
#[derive(Debug, Error)]
pub enum SyncError {
    /// Storage error
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    /// Network offline
    #[error("Network offline")]
    Offline,
    /// Sync conflict
    #[error("Sync conflict: {0}")]
    Conflict(String),
    /// Queue error
    #[error("Queue error: {0}")]
    Queue(String),
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    /// Local database error
    #[error("Local database error: {0}")]
    LocalDb(String),
}

/// Sync status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncStatus {
    /// Not syncing
    Idle,
    /// Syncing in progress
    Syncing,
    /// Waiting for network
    Offline,
    /// Sync error occurred
    Error,
    /// Up to date
    Synced,
}

/// Sync operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncOperation {
    /// Create new item
    Create {
        item_type: StorageItemType,
        local_id: String,
        data: Vec<u8>,
        tags: HashMap<String, String>,
    },
    /// Update existing item
    Update {
        item_id: String,
        local_id: String,
        data: Vec<u8>,
        expected_version: u64,
    },
    /// Delete item
    Delete {
        item_id: String,
        local_id: String,
    },
}

impl SyncOperation {
    /// Get the local ID for this operation
    pub fn local_id(&self) -> &str {
        match self {
            SyncOperation::Create { local_id, .. } => local_id,
            SyncOperation::Update { local_id, .. } => local_id,
            SyncOperation::Delete { local_id, .. } => local_id,
        }
    }
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Keep local version
    KeepLocal,
    /// Keep remote version
    KeepRemote,
    /// Keep both (create duplicate)
    KeepBoth,
    /// Manual resolution required
    Manual,
}

/// Sync conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    /// Conflict ID
    pub id: String,
    /// Local item ID
    pub local_id: String,
    /// Remote item ID
    pub remote_id: String,
    /// Local version
    pub local_version: u64,
    /// Remote version
    pub remote_version: u64,
    /// Local data
    pub local_data: Vec<u8>,
    /// Remote data
    pub remote_data: Vec<u8>,
    /// When conflict was detected
    pub detected_at: chrono::DateTime<chrono::Utc>,
}

/// Sync queue item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    /// Unique queue ID
    pub id: String,
    /// Operation to perform
    pub operation: SyncOperation,
    /// Number of retry attempts
    pub attempts: u32,
    /// When the item was queued
    pub queued_at: chrono::DateTime<chrono::Utc>,
    /// Last attempt time
    pub last_attempt: Option<chrono::DateTime<chrono::Utc>>,
    /// Last error message
    pub last_error: Option<String>,
}

/// Sync state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    /// Last successful sync time
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    /// Local to remote ID mapping
    pub id_mapping: HashMap<String, String>,
    /// Remote to local ID mapping
    pub reverse_mapping: HashMap<String, String>,
    /// Version tracking per item
    pub versions: HashMap<String, u64>,
    /// Items pending sync
    pub pending_count: usize,
}

impl Default for SyncState {
    fn default() -> Self {
        Self {
            last_sync: None,
            id_mapping: HashMap::new(),
            reverse_mapping: HashMap::new(),
            versions: HashMap::new(),
            pending_count: 0,
        }
    }
}

/// Sync manager configuration
#[derive(Debug, Clone)]
pub struct SyncConfig {
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Retry delay (exponential backoff base)
    pub retry_delay_ms: u64,
    /// Sync interval in seconds
    pub sync_interval_secs: u64,
    /// Maximum queue size
    pub max_queue_size: usize,
    /// Default conflict resolution
    pub default_conflict_resolution: ConflictResolution,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay_ms: 1000,
            sync_interval_secs: 60,
            max_queue_size: 1000,
            default_conflict_resolution: ConflictResolution::KeepLocal,
        }
    }
}

/// Sync event
#[derive(Debug, Clone)]
pub enum SyncEvent {
    /// Sync started
    Started,
    /// Sync completed
    Completed { uploaded: usize, downloaded: usize },
    /// Sync failed
    Failed(String),
    /// Item synced
    ItemSynced { local_id: String, remote_id: String },
    /// Conflict detected
    Conflict(SyncConflict),
    /// Network status changed
    NetworkChanged { online: bool },
    /// Progress update
    Progress { current: usize, total: usize },
}
