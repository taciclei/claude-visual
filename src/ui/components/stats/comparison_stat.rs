//! ComparisonStat component

use gpui::prelude::*;
use gpui::*;

/// Comparison stat
#[derive(Clone)]
pub struct ComparisonStat {
    label: String,
    current: String,
    previous: String,
    change_percent: f32,
}

impl ComparisonStat {
    pub fn new(
        label: impl Into<String>,
        current: impl Into<String>,
        previous: impl Into<String>,
        change_percent: f32,
    ) -> Self {
        Self {
            label: label.into(),
            current: current.into(),
            previous: previous.into(),
            change_percent,
        }
    }
}

impl RenderOnce for ComparisonStat {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let success = hsla(0.38, 0.7, 0.45, 1.0);
        let error = hsla(0.0, 0.7, 0.5, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        let is_positive = self.change_percent >= 0.0;
        let trend_color = if is_positive { success } else { error };
        let trend_icon = if is_positive { "↑" } else { "↓" };

        div()
            .flex()
            .flex_col()
            .gap_2()
            // Label
            .child(
                div()
                    .text_xs()
                    .text_color(text_muted)
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.label),
            )
            // Values row
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    // Current
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_xs().text_color(text_muted).child("Current"))
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(text)
                                    .child(self.current),
                            ),
                    )
                    // Divider
                    .child(div().w(px(1.0)).h(px(32.0)).bg(border))
                    // Previous
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_xs().text_color(text_muted).child("Previous"))
                            .child(div().text_lg().text_color(text_muted).child(self.previous)),
                    )
                    // Change
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(2.0))
                            .text_sm()
                            .text_color(trend_color)
                            .child(trend_icon)
                            .child(format!("{:.1}%", self.change_percent.abs())),
                    ),
            )
    }
}
