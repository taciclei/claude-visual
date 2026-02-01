//! Performance section rendering

use gpui::*;
use gpui::prelude::*;

use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Renders the performance section
    pub(crate) fn render_performance_section(
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
                    .child("PERFORMANCE")
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
                    .child(self.render_detail_row("Last Speed", &self.format_streaming_speed(), &theme))
                    .child(self.render_detail_row("Peak Speed", &format!("{:.1} tok/s", self.streaming.peak_speed), &theme))
                    .when_some(self.streaming.last_response_time_ms, |d, ms| {
                        d.child(self.render_detail_row("Last Response", &format!("{} ms", ms), &theme))
                    })
            )
    }
}
