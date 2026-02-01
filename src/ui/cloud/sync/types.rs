//! Event types for sync UI

use crate::cloud::ConflictResolution;

/// Events emitted by the sync status panel
pub enum SyncStatusPanelEvent {
    /// Trigger manual sync
    TriggerSync,
    /// Open conflict resolution
    OpenConflicts,
    /// Resolve a conflict
    ResolveConflict {
        conflict_id: String,
        resolution: ConflictResolution,
    },
    /// Toggle auto-sync
    ToggleAutoSync,
    /// Change sync interval
    SetSyncInterval(u64),
}
