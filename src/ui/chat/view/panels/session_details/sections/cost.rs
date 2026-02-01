//! Cost section rendering

use gpui::*;
use gpui::prelude::*;

use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Renders the cost section
    pub(crate) fn render_cost_section(
        &self,
        theme: &crate::app::theme::Theme,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.text_muted)
                    .child("COST")
            )
            .child(
                div()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(theme.colors.background)
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(self.render_detail_row("Session Cost", &format!("${:.4}", self.stats.cost), &theme))
                    .child(self.render_detail_row("API Requests", &format!("{}", self.stats.total_api_requests), &theme))
            )
    }
}
