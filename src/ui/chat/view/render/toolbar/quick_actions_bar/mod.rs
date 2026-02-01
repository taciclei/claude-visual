//! Quick actions bar render function

mod session_skills;
mod quick_actions;
mod keyboard_hints;
mod panel_buttons;
mod status_indicators;

use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;
use crate::ui::chat::view::core::ChatView;
use crate::ui::chat::view::types::ChatViewEvent;

impl ChatView {
    pub fn render_quick_actions_bar(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .items_center()
            .gap_1()
            .px_4()
            .py_1()
            .bg(theme.colors.surface.opacity(0.5))
            .border_t_1()
            .border_color(theme.colors.border.opacity(0.5))
            // Session skills section
            .child(self.render_session_skills(theme, cx))
            // Quick action buttons
            .child(self.render_quick_action_buttons(theme, cx))
            // Separator
            .child(self.render_separator(theme))
            // Keyboard shortcuts
            .child(self.render_keyboard_hints(theme))
            // Recent commands
            .child(self.render_recent_commands(theme, cx))
            // Draft indicator
            .when(self.has_draft(), |d| {
                d.child(self.render_separator(theme))
                .child(self.render_draft_indicator(&theme))
            })
            // Panel buttons
            .child(self.render_panel_buttons(theme, cx))
            // Status indicators
            .child(self.render_status_indicators(theme, cx))
    }

    fn render_separator(&self, theme: &crate::app::theme::Theme) -> Div {
        div()
            .w(px(1.0))
            .h(px(12.0))
            .bg(theme.colors.border)
            .mx_1()
    }
}
