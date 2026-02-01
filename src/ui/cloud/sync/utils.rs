//! Shared utility functions for sync UI

use std::sync::Arc;

use gpui::*;

use crate::app::state::AppState;
use crate::cloud::SyncStatus;

/// Get status color for a given sync status
pub(super) fn status_color(status: SyncStatus, app_state: &Arc<AppState>, cx: &App) -> Hsla {
    let theme = app_state.theme.read(cx);
    match status {
        SyncStatus::Idle => theme.colors.text_muted,
        SyncStatus::Syncing => theme.colors.accent,
        SyncStatus::Offline => theme.colors.warning,
        SyncStatus::Error => theme.colors.error,
        SyncStatus::Synced => theme.colors.success,
    }
}

/// Get status text for a given sync status
pub(super) fn status_text(status: SyncStatus, is_panel: bool) -> &'static str {
    match status {
        SyncStatus::Idle => "Idle",
        SyncStatus::Syncing => "Syncing...",
        SyncStatus::Offline => "Offline",
        SyncStatus::Error => {
            if is_panel {
                "Sync Error"
            } else {
                "Error"
            }
        }
        SyncStatus::Synced => {
            if is_panel {
                "Up to date"
            } else {
                "Synced"
            }
        }
    }
}

/// Format last sync time as human-readable string
pub(super) fn format_last_sync(
    last_sync: Option<chrono::DateTime<chrono::Utc>>,
    is_panel: bool,
) -> String {
    if let Some(time) = last_sync {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(time);

        if duration.num_seconds() < 60 {
            "Just now".to_string()
        } else if duration.num_minutes() < 60 {
            let minutes = duration.num_minutes();
            if is_panel {
                format!("{} minutes ago", minutes)
            } else {
                format!("{} min ago", minutes)
            }
        } else if duration.num_hours() < 24 {
            format!("{} hours ago", duration.num_hours())
        } else {
            format!("{} days ago", duration.num_days())
        }
    } else {
        if is_panel {
            "Never synced".to_string()
        } else {
            "Never".to_string()
        }
    }
}
