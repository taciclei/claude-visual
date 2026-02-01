use super::types::*;
use super::manager::SyncManager;

impl SyncManager {
    /// Resolve a conflict
    pub async fn resolve_conflict(
        &self,
        conflict_id: &str,
        resolution: ConflictResolution,
    ) -> Result<(), SyncError> {
        let mut conflicts = self.conflicts.write().await;
        let index = conflicts
            .iter()
            .position(|c| c.id == conflict_id)
            .ok_or_else(|| SyncError::Conflict(format!("Conflict {} not found", conflict_id)))?;

        let conflict = conflicts.remove(index);

        match resolution {
            ConflictResolution::KeepLocal => {
                // Queue local version for upload
                self.queue_operation(SyncOperation::Update {
                    item_id: conflict.remote_id,
                    local_id: conflict.local_id,
                    data: conflict.local_data,
                    expected_version: conflict.remote_version,
                })
                .await?;
            }
            ConflictResolution::KeepRemote => {
                // Update local with remote
                // TODO: Update local database with conflict.remote_data
                let mut state = self.state.write().await;
                state
                    .versions
                    .insert(conflict.local_id, conflict.remote_version);
            }
            ConflictResolution::KeepBoth => {
                // Create duplicate
                // TODO: Create duplicate in local database
            }
            ConflictResolution::Manual => {
                // Put conflict back
                conflicts.push(conflict);
            }
        }

        Ok(())
    }
}
