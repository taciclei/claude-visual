//! Activity panel events

use gpui::*;

use crate::cloud::team::ActivityTarget;

/// Events emitted by the activity panel
pub enum ActivityPanelEvent {
    /// Navigate to activity target
    NavigateToTarget {
        target_type: ActivityTarget,
        target_id: String,
    },
    /// Refresh activity feed
    Refresh,
    /// Panel closed
    Closed,
}

impl EventEmitter<ActivityPanelEvent> for super::ActivityPanel {}
