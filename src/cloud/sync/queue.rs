use super::types::*;
use super::manager::SyncManager;

impl SyncManager {
    /// Queue an operation
    pub async fn queue_operation(&self, operation: SyncOperation) -> Result<String, SyncError> {
        let mut queue = self.queue.write().await;

        // Check queue size
        if queue.len() >= self.config.max_queue_size {
            return Err(SyncError::Queue("Queue is full".to_string()));
        }

        let item = QueueItem {
            id: uuid::Uuid::new_v4().to_string(),
            operation,
            attempts: 0,
            queued_at: chrono::Utc::now(),
            last_attempt: None,
            last_error: None,
        };

        let id = item.id.clone();
        queue.push_back(item);

        // Update pending count
        self.state.write().await.pending_count = queue.len();

        // Persist queue
        drop(queue);
        self.persist_queue().await?;

        // Trigger sync if online
        if *self.is_online.read().await {
            tokio::spawn({
                let manager = SyncManager::clone_arc(self);
                async move {
                    let _ = manager.sync().await;
                }
            });
        }

        Ok(id)
    }

    /// Process the sync queue
    pub(super) async fn process_queue(&self, uploaded: &mut usize) -> Result<(), SyncError> {
        let mut to_retry = Vec::new();
        let total = self.queue.read().await.len();
        let mut current = 0;

        while let Some(mut item) = self.queue.write().await.pop_front() {
            current += 1;
            let _ = self.event_tx.send(SyncEvent::Progress { current, total });

            match self.process_item(&item).await {
                Ok(remote_id) => {
                    *uploaded += 1;

                    // Update mappings
                    let local_id = item.operation.local_id().to_string();
                    let mut state = self.state.write().await;
                    state.id_mapping.insert(local_id.clone(), remote_id.clone());
                    state.reverse_mapping.insert(remote_id.clone(), local_id.clone());

                    let _ = self.event_tx.send(SyncEvent::ItemSynced {
                        local_id,
                        remote_id,
                    });
                }
                Err(e) => {
                    item.attempts += 1;
                    item.last_attempt = Some(chrono::Utc::now());
                    item.last_error = Some(e.to_string());

                    if item.attempts < self.config.max_retries {
                        to_retry.push(item);
                    } else {
                        tracing::error!(
                            "Failed to sync item {} after {} attempts: {}",
                            item.id,
                            item.attempts,
                            e
                        );
                    }
                }
            }
        }

        // Re-queue failed items
        if !to_retry.is_empty() {
            let mut queue = self.queue.write().await;
            for item in to_retry {
                queue.push_back(item);
            }
        }

        // Update pending count and persist
        let queue_len = self.queue.read().await.len();
        self.state.write().await.pending_count = queue_len;
        self.persist_queue().await?;

        Ok(())
    }

    /// Process a single queue item
    pub(super) async fn process_item(&self, item: &QueueItem) -> Result<String, SyncError> {
        let storage = self.storage.read().await;

        match &item.operation {
            SyncOperation::Create {
                item_type,
                data,
                tags,
                ..
            } => {
                let metadata = storage
                    .upload(*item_type, data, tags.clone())
                    .await
                    .map_err(SyncError::Storage)?;
                Ok(metadata.id)
            }
            SyncOperation::Update {
                item_id,
                data,
                expected_version,
                ..
            } => {
                let metadata = storage
                    .update(item_id, data, *expected_version)
                    .await
                    .map_err(SyncError::Storage)?;
                Ok(metadata.id)
            }
            SyncOperation::Delete { item_id, .. } => {
                storage.delete(item_id).await.map_err(SyncError::Storage)?;
                Ok(item_id.clone())
            }
        }
    }
}
