//! Session history management methods

use gpui::*;

use super::super::core::ChatView;
use super::super::types::{ChatViewEvent, NotificationType, RecentSession};

impl ChatView {
    /// Toggle session history panel
    pub fn toggle_session_history(&mut self, cx: &mut Context<Self>) {
        self.panels.session_history = !self.panels.session_history;
        cx.notify();
    }

    /// Add a session to recent history
    pub fn add_recent_session(&mut self, session: RecentSession, cx: &mut Context<Self>) {
        // Remove if exists (to move to front)
        self.recent_sessions.retain(|s| s.session_id != session.session_id);
        // Add to front
        self.recent_sessions.insert(0, session);
        // Keep only last 10
        self.recent_sessions.truncate(10);
        cx.notify();
    }

    /// Resume a previous session
    pub fn resume_session(&mut self, session_id: &str, cx: &mut Context<Self>) {
        self.panels.session_history = false;
        cx.emit(ChatViewEvent::Submit(format!("/resume {}", session_id)));
    }

    /// Resume the most recent session
    pub fn resume_last_session(&mut self, cx: &mut Context<Self>) {
        if let Some(session) = self.recent_sessions.first() {
            let session_id = session.session_id.clone();
            self.show_notification(&format!("Resuming session: {}", session.title), NotificationType::Info, cx);
            cx.emit(ChatViewEvent::Submit(format!("/resume {}", session_id)));
        } else {
            self.show_notification("No recent sessions to resume", NotificationType::Warning, cx);
        }
    }
}
