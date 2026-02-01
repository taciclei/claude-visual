//! Sync status indicator component

use std::sync::Arc;

use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;
use crate::cloud::SyncStatus;

use super::utils;

/// Sync status indicator (small, for toolbar/statusbar)
pub struct SyncStatusIndicator {
    pub(crate) app_state: Arc<AppState>,
    /// Current sync status
    pub(crate) status: SyncStatus,
    /// Last sync time
    pub(crate) last_sync: Option<chrono::DateTime<chrono::Utc>>,
    /// Pending items count
    pub(crate) pending_count: usize,
    /// Conflict count
    pub(crate) conflict_count: usize,
}

impl SyncStatusIndicator {
    /// Create a new sync status indicator
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self {
            app_state,
            status: SyncStatus::Idle,
            last_sync: None,
            pending_count: 0,
            conflict_count: 0,
        }
    }

    /// Update status
    pub fn set_status(&mut self, status: SyncStatus) {
        self.status = status;
    }

    /// Update last sync time
    pub fn set_last_sync(&mut self, time: Option<chrono::DateTime<chrono::Utc>>) {
        self.last_sync = time;
    }

    /// Update pending count
    pub fn set_pending_count(&mut self, count: usize) {
        self.pending_count = count;
    }

    /// Update conflict count
    pub fn set_conflict_count(&mut self, count: usize) {
        self.conflict_count = count;
    }
}

impl RenderOnce for SyncStatusIndicator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let status_color = utils::status_color(self.status, &self.app_state, cx);
        let status_text = utils::status_text(self.status, false);

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .px_2()
            .py_1()
            .rounded_md()
            .cursor_pointer()
            .hover(|this| this.bg(theme.colors.surface_hover))
            // Status dot
            .child(
                div()
                    .size_2()
                    .rounded_full()
                    .bg(status_color)
                    .when(matches!(self.status, SyncStatus::Syncing), |this| {
                        // Pulse animation would go here
                        this
                    }),
            )
            // Status text
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(status_text),
            )
            // Pending count badge
            .when(self.pending_count > 0, |this| {
                this.child(
                    div()
                        .px_1p5()
                        .py_0p5()
                        .rounded_full()
                        .bg(theme.colors.accent.opacity(0.2))
                        .text_xs()
                        .text_color(theme.colors.accent)
                        .child(format!("{}", self.pending_count)),
                )
            })
            // Conflict indicator
            .when(self.conflict_count > 0, |this| {
                this.child(
                    div()
                        .px_1p5()
                        .py_0p5()
                        .rounded_full()
                        .bg(theme.colors.warning.opacity(0.2))
                        .text_xs()
                        .text_color(theme.colors.warning)
                        .child(format!("! {}", self.conflict_count)),
                )
            })
    }
}
