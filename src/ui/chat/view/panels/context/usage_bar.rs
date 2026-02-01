//! Context panel usage bar component

use gpui::prelude::*;
use gpui::*;

use super::super::super::core::ChatView;
use crate::app::theme::Theme;
use crate::ui::pct;

impl ChatView {
    pub(super) fn render_usage_bar(&self, theme: &Theme) -> impl IntoElement {
        let session_info = self.session_info.as_ref();
        let file_count = self.context_files.len();
        let tool_count = session_info.map(|i| i.tools.len()).unwrap_or(0);
        let usage_pct = self.context_usage_percentage();
        let usage_color = if usage_pct < 0.5 {
            theme.colors.success
        } else if usage_pct < 0.8 {
            theme.colors.warning
        } else {
            theme.colors.error
        };

        div()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(theme.colors.border)
            .bg(theme.colors.background.opacity(0.5))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .mb_2()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Context Window Usage"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(usage_color)
                            .child(format!("{}%", (usage_pct * 100.0) as u32)),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .h(px(6.0))
                    .rounded_full()
                    .bg(theme.colors.border)
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .w(pct((usage_pct * 100.0) as f32))
                            .bg(usage_color)
                            .rounded_full(),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .mt_2()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(self.format_context_usage())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(format!("{} files", file_count))
                            .child(format!("{} tools", tool_count)),
                    ),
            )
    }
}
