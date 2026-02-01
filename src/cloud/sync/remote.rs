use super::manager::SyncManager;
use super::types::*;
use crate::cloud::storage::{StorageItemType, StorageMetadata};

impl SyncManager {
    /// Fetch remote changes
    pub(super) async fn fetch_remote_changes(
        &self,
        downloaded: &mut usize,
    ) -> Result<(), SyncError> {
        let storage = self.storage.read().await;
        let state = self.state.read().await;

        // Fetch conversations
        let remote_items = storage
            .list_items(StorageItemType::Conversation)
            .await
            .map_err(SyncError::Storage)?;

        for remote in remote_items {
            // Check if we have this item locally
            if let Some(local_id) = state.reverse_mapping.get(&remote.id) {
                // Check version
                if let Some(&local_version) = state.versions.get(local_id) {
                    if remote.version > local_version {
                        // Remote is newer - download and merge
                        match self.download_and_merge(&remote, local_id).await {
                            Ok(()) => *downloaded += 1,
                            Err(SyncError::Conflict(msg)) => {
                                tracing::warn!("Conflict detected: {}", msg);
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }
            } else {
                // New remote item - download
                match self.download_new_item(&remote).await {
                    Ok(()) => *downloaded += 1,
                    Err(e) => tracing::warn!("Failed to download item {}: {}", remote.id, e),
                }
            }
        }

        Ok(())
    }

    /// Download and merge a remote item
    pub(super) async fn download_and_merge(
        &self,
        remote: &StorageMetadata,
        local_id: &str,
    ) -> Result<(), SyncError> {
        let storage = self.storage.read().await;
        let remote_data = storage
            .download(&remote.id)
            .await
            .map_err(SyncError::Storage)?;

        // Get local data (would be from local database)
        // For now, we'll just accept remote version based on config
        match self.config.default_conflict_resolution {
            ConflictResolution::KeepRemote => {
                // Update local with remote data
                tracing::info!("Accepting remote version for {}", local_id);
                // TODO: Update local database
                let mut state = self.state.write().await;
                state.versions.insert(local_id.to_string(), remote.version);
                Ok(())
            }
            ConflictResolution::KeepLocal => {
                // Queue local version for upload
                tracing::info!("Keeping local version for {}", local_id);
                Ok(())
            }
            ConflictResolution::Manual => {
                // Create conflict
                let conflict = SyncConflict {
                    id: uuid::Uuid::new_v4().to_string(),
                    local_id: local_id.to_string(),
                    remote_id: remote.id.clone(),
                    local_version: self
                        .state
                        .read()
                        .await
                        .versions
                        .get(local_id)
                        .copied()
                        .unwrap_or(0),
                    remote_version: remote.version,
                    local_data: Vec::new(), // Would be loaded from local DB
                    remote_data,
                    detected_at: chrono::Utc::now(),
                };

                self.conflicts.write().await.push(conflict.clone());
                let _ = self.event_tx.send(SyncEvent::Conflict(conflict.clone()));

                Err(SyncError::Conflict(format!(
                    "Conflict for item {}",
                    local_id
                )))
            }
            ConflictResolution::KeepBoth => {
                // Create duplicate locally
                tracing::info!("Creating duplicate for conflict on {}", local_id);
                // TODO: Create duplicate in local database
                Ok(())
            }
        }
    }

    /// Download a new remote item
    pub(super) async fn download_new_item(
        &self,
        remote: &StorageMetadata,
    ) -> Result<(), SyncError> {
        let storage = self.storage.read().await;
        let data = storage
            .download(&remote.id)
            .await
            .map_err(SyncError::Storage)?;

        // Generate local ID
        let local_id = uuid::Uuid::new_v4().to_string();

        // TODO: Save to local database

        // Update mappings
        let mut state = self.state.write().await;
        state.id_mapping.insert(local_id.clone(), remote.id.clone());
        state
            .reverse_mapping
            .insert(remote.id.clone(), local_id.clone());
        state.versions.insert(local_id.clone(), remote.version);

        tracing::info!("Downloaded new item {} ({} bytes)", remote.id, data.len());

        Ok(())
    }
}
