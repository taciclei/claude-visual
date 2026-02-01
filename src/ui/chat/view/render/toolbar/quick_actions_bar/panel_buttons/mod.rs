//! Panel toggle buttons rendering - modular organization

mod utility_buttons;
mod permissions_button;
mod input_panels;
mod content_panels;
mod collections;
mod info_panels;
mod mode_toggles;
mod settings_buttons;
mod task_button;
mod git_button;
mod bookmarks_button;

use gpui::*;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Render all panel buttons in a horizontal layout
    pub(super) fn render_panel_buttons(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let border_color = theme.colors.border.opacity(0.5);

        div()
            .flex()
            .items_center()
            .gap_1()
            // Utility buttons
            .child(self.render_suggestions_button(theme, cx))
            .child(self.render_session_history_button(theme, cx))
            // Permissions (if pending)
            .children(self.render_permissions_button(theme, cx))
            // Separator before panels section
            .child(
                div()
                    .w(px(1.0))
                    .h(px(12.0))
                    .bg(border_color)
                    .mx_1()
            )
            // Input panels
            .child(self.render_file_picker_button(theme, cx))
            .child(self.render_commands_panel_button(theme, cx))
            .child(self.render_templates_panel_button(theme, cx))
            // Content panels
            .child(self.render_context_panel_button(theme, cx))
            .child(self.render_export_panel_button(theme, cx))
            .child(self.render_notes_panel_button(theme, cx))
            // Collections
            .child(self.render_favorites_panel_button(theme, cx))
            .child(self.render_pinned_panel_button(theme, cx))
            .child(self.render_recent_files_panel_button(theme, cx))
            // Info panels
            .child(self.render_stats_panel_button(theme, cx))
            .child(self.render_focus_mode_button(theme, cx))
            .child(self.render_tags_panel_button(theme, cx))
            // Settings
            .child(self.render_quick_settings_button(theme, cx))
            .child(self.render_mcp_panel_button(theme, cx))
            // Tasks
            .child(self.render_tasks_panel_button(theme, cx))
            // Git (conditional)
            .children(self.render_git_panel_button(theme, cx))
            // Bookmarks (conditional)
            .children(self.render_bookmarks_button(theme, cx))
    }
}
