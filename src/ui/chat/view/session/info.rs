//! Session info methods

use gpui::*;

use super::super::core::ChatView;

impl ChatView {
    /// Get current session ID for session continuity
    pub fn current_session_id(&self) -> Option<String> {
        self.session_info
            .as_ref()
            .map(|info| info.session_id.clone())
            .filter(|id| !id.is_empty())
    }

    /// Toggle session details panel
    pub fn toggle_session_details(&mut self, cx: &mut Context<Self>) {
        self.panels.session_details = !self.panels.session_details;
        cx.notify();
    }
}
