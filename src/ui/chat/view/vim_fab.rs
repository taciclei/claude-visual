//! Vim mode and FAB (Floating Action Button) menu methods

use super::core::ChatView;
use gpui::Context;

impl ChatView {
    /// Toggle floating action button menu
    pub fn toggle_fab_menu(&mut self, cx: &mut Context<Self>) {
        self.panels.fab_menu = !self.panels.fab_menu;
        cx.notify();
    }

    /// Toggle vim mode in the input
    pub fn toggle_vim_mode(&mut self, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| {
            input.toggle_vim_mode(cx);
        });
    }

    /// Check if vim mode is enabled
    pub fn is_vim_mode_enabled(&self, cx: &Context<ChatView>) -> bool {
        self.input.read(cx).has_vim_state()
    }

    /// Track a command as recently used
    pub(crate) fn track_recent_command(&mut self, command: &str) {
        // Only track slash commands
        if !command.starts_with('/') {
            return;
        }

        // Remove if already exists (to move to front)
        self.palette.recent_commands.retain(|c| c != command);

        // Add to front
        self.palette.recent_commands.insert(0, command.to_string());

        // Keep only last 10
        self.palette.recent_commands.truncate(10);
    }

    /// Start tracking response time
    pub(crate) fn start_response_timer(&mut self) {
        self.streaming.response_start_time = Some(chrono::Utc::now());
    }

    /// Stop tracking response time and record duration
    pub(crate) fn stop_response_timer(&mut self) {
        if let Some(start) = self.streaming.response_start_time.take() {
            let duration = chrono::Utc::now().signed_duration_since(start);
            self.streaming.last_response_time_ms = Some(duration.num_milliseconds() as u64);
        }
    }

    /// Clear the highlight (called after animation completes or user scrolls)
    pub fn clear_highlight(&mut self) {
        self.highlighted_message = None;
    }
}
