//! Debug toolbar rendering

mod buttons;
mod stepping;
mod ai_assist;
mod status;

use gpui::*;
use gpui::prelude::*;

use crate::ui::debug::debug_panel::DebugPanel;
use crate::debug::DebugState;

impl DebugPanel {
    /// Render toolbar
    pub(in crate::ui::debug::debug_panel) fn render_toolbar(&self, theme: &crate::app::theme::Theme, cx: &Context<Self>) -> impl IntoElement {
        let state = self.state;
        let is_running = state == DebugState::Running;
        let is_stopped = state == DebugState::Stopped || state == DebugState::Paused;
        let is_idle = state == DebugState::Idle || state == DebugState::Terminated;

        // Copy theme colors for move closures
        let success_color = theme.colors.success;
        let error_color = theme.colors.error;
        let warning_color = theme.colors.warning;
        let surface_color = theme.colors.surface;
        let border_color = theme.colors.border;
        let text_color = theme.colors.text;
        let text_muted_color = theme.colors.text_muted;
        let accent_color = theme.colors.accent;

        div()
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_1()
            .bg(surface_color)
            .border_b_1()
            .border_color(border_color)
            // Start/Stop button
            .child(self.render_start_stop_button(is_idle, success_color, error_color, cx))
            // Restart button
            .when(!is_idle, |d| {
                d.child(self.render_restart_button(surface_color, border_color, warning_color, cx))
            })
            // Separator
            .child(Self::render_separator(border_color))
            // Continue button
            .child(self.render_continue_button(is_stopped, success_color, cx))
            // Pause button
            .child(self.render_pause_button(is_running, warning_color, cx))
            // Separator
            .child(Self::render_separator(border_color))
            // Step Over
            .child(self.render_step_over_button(is_stopped, surface_color, border_color, text_color, cx))
            // Step Into
            .child(self.render_step_into_button(is_stopped, surface_color, border_color, text_color, cx))
            // Step Out
            .child(self.render_step_out_button(is_stopped, surface_color, border_color, text_color, cx))
            // Spacer
            .child(div().flex_1())
            // AI Assistant button
            .child(self.render_ai_assist_button(surface_color, accent_color, cx))
            // Separator
            .child(Self::render_separator(border_color))
            // Status
            .child(self.render_status(state, text_muted_color))
    }
}
