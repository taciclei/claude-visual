//! Core update notification component logic

use gpui::*;
use std::sync::Arc;

use crate::app::state::AppState;
use crate::update::{InstallProgress, UpdateStatus};

/// Update notification component
pub struct UpdateNotification {
    pub(crate) app_state: Arc<AppState>,
    pub(crate) update_status: UpdateStatus,
    pub(crate) install_progress: Option<InstallProgress>,
    pub(crate) dismissed: bool,
    pub(crate) skipped_version: Option<String>,
    pub(crate) expanded: bool,
    pub(crate) focus_handle: FocusHandle,
}

impl UpdateNotification {
    /// Create a new update notification
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            update_status: UpdateStatus::Unknown,
            install_progress: None,
            dismissed: false,
            skipped_version: None,
            expanded: false,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Set the update status
    pub fn set_status(&mut self, status: UpdateStatus, cx: &mut Context<Self>) {
        // Don't show if skipped version matches
        if let UpdateStatus::UpdateAvailable(ref info) = status {
            if self.skipped_version.as_deref() == Some(&info.version) {
                return;
            }
        }

        self.update_status = status;
        self.dismissed = false;
        cx.notify();
    }

    /// Set the install progress
    pub fn set_progress(&mut self, progress: InstallProgress, cx: &mut Context<Self>) {
        self.install_progress = Some(progress);
        cx.notify();
    }

    /// Check if the notification should be visible
    pub fn is_visible(&self) -> bool {
        if self.dismissed {
            return false;
        }

        matches!(
            &self.update_status,
            UpdateStatus::UpdateAvailable(_) | UpdateStatus::Error(_)
        ) || self.install_progress.is_some()
    }

    /// Skip the current version
    pub fn skip_version(&mut self, version: String, cx: &mut Context<Self>) {
        self.skipped_version = Some(version);
        self.dismissed = true;
        cx.notify();
    }

    /// Dismiss the notification temporarily
    pub fn dismiss(&mut self, cx: &mut Context<Self>) {
        self.dismissed = true;
        cx.notify();
    }

    /// Toggle expanded view
    pub(crate) fn toggle_expanded(&mut self, cx: &mut Context<Self>) {
        self.expanded = !self.expanded;
        cx.notify();
    }
}
