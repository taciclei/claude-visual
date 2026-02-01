use gpui::*;
use gpui::prelude::*;
use super::super::SettingsModal;

impl SettingsModal {
    /// Render the git tab
    pub(crate) fn render_git_tab(&self, cx: &Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(self.render_section(
                "Git Integration",
                "Git-related settings",
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(self.app_state.theme.read(cx).colors.text_muted)
                            .child("Git integration is automatic when a project is a git repository."),
                    ),
                cx,
            ))
    }
}
