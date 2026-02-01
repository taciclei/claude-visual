use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use super::types::*;
use crate::cloud::storage::CloudStorage;

/// Sync manager for offline-first synchronization
pub struct SyncManager {
    /// Cloud storage client
    pub(super) storage: Arc<RwLock<CloudStorage>>,
    /// Sync queue
    pub(super) queue: Arc<RwLock<VecDeque<QueueItem>>>,
    /// Current status
    pub(super) status: Arc<RwLock<SyncStatus>>,
    /// Sync state
    pub(super) state: Arc<RwLock<SyncState>>,
    /// Pending conflicts
    pub(super) conflicts: Arc<RwLock<Vec<SyncConflict>>>,
    /// Configuration
    pub(super) config: SyncConfig,
    /// Event sender
    pub(super) event_tx: mpsc::UnboundedSender<SyncEvent>,
    /// Event receiver
    pub(super) event_rx: Arc<RwLock<mpsc::UnboundedReceiver<SyncEvent>>>,
    /// Queue persistence path
    pub(super) queue_path: PathBuf,
    /// State persistence path
    pub(super) state_path: PathBuf,
    /// Is syncing
    pub(super) is_syncing: Arc<RwLock<bool>>,
    /// Is online
    pub(super) is_online: Arc<RwLock<bool>>,
}

impl SyncManager {
    /// Create a new sync manager
    pub fn new(storage: CloudStorage, data_dir: PathBuf, config: SyncConfig) -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        Self {
            storage: Arc::new(RwLock::new(storage)),
            queue: Arc::new(RwLock::new(VecDeque::new())),
            status: Arc::new(RwLock::new(SyncStatus::Idle)),
            state: Arc::new(RwLock::new(SyncState::default())),
            conflicts: Arc::new(RwLock::new(Vec::new())),
            config,
            event_tx,
            event_rx: Arc::new(RwLock::new(event_rx)),
            queue_path: data_dir.join("sync_queue.json"),
            state_path: data_dir.join("sync_state.json"),
            is_syncing: Arc::new(RwLock::new(false)),
            is_online: Arc::new(RwLock::new(true)),
        }
    }

    /// Initialize sync manager, loading persisted state
    pub async fn initialize(&self) -> Result<(), SyncError> {
        // Load queue from disk
        if self.queue_path.exists() {
            let data = tokio::fs::read_to_string(&self.queue_path)
                .await
                .map_err(|e| SyncError::Queue(e.to_string()))?;
            let items: Vec<QueueItem> =
                serde_json::from_str(&data).map_err(|e| SyncError::Serialization(e.to_string()))?;
            let mut queue = self.queue.write().await;
            queue.extend(items);
        }

        // Load state from disk
        if self.state_path.exists() {
            let data = tokio::fs::read_to_string(&self.state_path)
                .await
                .map_err(|e| SyncError::Queue(e.to_string()))?;
            let state: SyncState =
                serde_json::from_str(&data).map_err(|e| SyncError::Serialization(e.to_string()))?;
            *self.state.write().await = state;
        }

        // Update pending count
        let queue_len = self.queue.read().await.len();
        self.state.write().await.pending_count = queue_len;

        Ok(())
    }

    /// Get current sync status
    pub async fn status(&self) -> SyncStatus {
        *self.status.read().await
    }

    /// Get sync state
    pub async fn state(&self) -> SyncState {
        self.state.read().await.clone()
    }

    /// Get pending conflicts
    pub async fn conflicts(&self) -> Vec<SyncConflict> {
        self.conflicts.read().await.clone()
    }

    /// Set online status
    pub async fn set_online(&self, online: bool) {
        *self.is_online.write().await = online;
        let _ = self.event_tx.send(SyncEvent::NetworkChanged { online });

        if online && !*self.is_syncing.read().await {
            // Trigger sync when coming online
            let _ = self.sync().await;
        } else if !online {
            *self.status.write().await = SyncStatus::Offline;
        }
    }

    /// Clone as Arc for spawning
    pub(super) fn clone_arc(&self) -> Arc<Self> {
        // This is a workaround - in real code, SyncManager should be wrapped in Arc
        // For now, we'll just return a new instance with shared state
        Arc::new(Self {
            storage: Arc::clone(&self.storage),
            queue: Arc::clone(&self.queue),
            status: Arc::clone(&self.status),
            state: Arc::clone(&self.state),
            conflicts: Arc::clone(&self.conflicts),
            config: self.config.clone(),
            event_tx: self.event_tx.clone(),
            event_rx: Arc::clone(&self.event_rx),
            queue_path: self.queue_path.clone(),
            state_path: self.state_path.clone(),
            is_syncing: Arc::clone(&self.is_syncing),
            is_online: Arc::clone(&self.is_online),
        })
    }

    /// Perform sync
    pub async fn sync(&self) -> Result<(), SyncError> {
        // Check if already syncing
        {
            let mut is_syncing = self.is_syncing.write().await;
            if *is_syncing {
                return Ok(());
            }
            *is_syncing = true;
        }

        // Check if online
        if !*self.is_online.read().await {
            *self.is_syncing.write().await = false;
            return Err(SyncError::Offline);
        }

        *self.status.write().await = SyncStatus::Syncing;
        let _ = self.event_tx.send(SyncEvent::Started);

        let mut uploaded = 0;
        let mut downloaded = 0;

        // Process queue
        let result = self.process_queue(&mut uploaded).await;

        // Fetch remote changes
        if result.is_ok() {
            if let Err(e) = self.fetch_remote_changes(&mut downloaded).await {
                tracing::warn!("Failed to fetch remote changes: {}", e);
            }
        }

        // Update status
        *self.is_syncing.write().await = false;

        match result {
            Ok(()) => {
                *self.status.write().await = SyncStatus::Synced;
                self.state.write().await.last_sync = Some(chrono::Utc::now());
                self.persist_state().await?;
                let _ = self.event_tx.send(SyncEvent::Completed {
                    uploaded,
                    downloaded,
                });
                Ok(())
            }
            Err(e) => {
                *self.status.write().await = SyncStatus::Error;
                let _ = self.event_tx.send(SyncEvent::Failed(e.to_string()));
                Err(e)
            }
        }
    }
}
