//! Sync status panel component

mod render;

use std::sync::Arc;

use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;
use crate::cloud::SyncStatus;

use super::types::SyncStatusPanelEvent;

/// Sync status panel (expanded view)
pub struct SyncStatusPanel {
    pub(crate) app_state: Arc<AppState>,
    /// Current sync status
    pub(crate) status: SyncStatus,
    /// Last sync time
    pub(crate) last_sync: Option<chrono::DateTime<chrono::Utc>>,
    /// Pending items count
    pub(crate) pending_count: usize,
    /// Conflict count
    pub(crate) conflict_count: usize,
    /// Upload count since last sync
    pub(crate) upload_count: usize,
    /// Download count since last sync
    pub(crate) download_count: usize,
    /// Auto-sync enabled
    pub(crate) auto_sync: bool,
    /// Sync interval in seconds
    pub(crate) sync_interval: u64,
    /// Is expanded
    pub(crate) expanded: bool,
    /// Focus handle
    focus_handle: FocusHandle,
}

impl EventEmitter<SyncStatusPanelEvent> for SyncStatusPanel {}

impl SyncStatusPanel {
    /// Create a new sync status panel
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            status: SyncStatus::Idle,
            last_sync: None,
            pending_count: 0,
            conflict_count: 0,
            upload_count: 0,
            download_count: 0,
            auto_sync: true,
            sync_interval: 60,
            expanded: true,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Update status
    pub fn set_status(&mut self, status: SyncStatus, cx: &mut Context<Self>) {
        self.status = status;
        cx.notify();
    }

    /// Update last sync time
    pub fn set_last_sync(
        &mut self,
        time: Option<chrono::DateTime<chrono::Utc>>,
        cx: &mut Context<Self>,
    ) {
        self.last_sync = time;
        cx.notify();
    }

    /// Update counts
    pub fn set_counts(
        &mut self,
        pending: usize,
        conflicts: usize,
        uploaded: usize,
        downloaded: usize,
        cx: &mut Context<Self>,
    ) {
        self.pending_count = pending;
        self.conflict_count = conflicts;
        self.upload_count = uploaded;
        self.download_count = downloaded;
        cx.notify();
    }

    /// Toggle expanded state
    pub fn toggle_expanded(&mut self, cx: &mut Context<Self>) {
        self.expanded = !self.expanded;
        cx.notify();
    }

    /// Toggle auto-sync
    pub fn toggle_auto_sync(&mut self, cx: &mut Context<Self>) {
        self.auto_sync = !self.auto_sync;
        cx.emit(SyncStatusPanelEvent::ToggleAutoSync);
        cx.notify();
    }
}

impl Focusable for SyncStatusPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SyncStatusPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .w_full()
            .bg(theme.colors.surface)
            .rounded_lg()
            .border_1()
            .border_color(theme.colors.border)
            .child(self.render_header(cx))
            .when(self.expanded, |this| {
                this.child(self.render_stats(cx))
                    .child(self.render_conflicts(cx))
                    .child(self.render_actions(cx))
            })
    }
}
