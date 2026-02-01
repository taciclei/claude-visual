//! MetricTrend component

use gpui::prelude::*;
use gpui::*;

/// Metric with trend
#[derive(Clone)]
pub struct MetricTrend {
    pub(crate) value: String,
    pub(crate) change: String,
    pub(crate) is_positive: bool,
    pub(crate) label: Option<String>,
}

impl MetricTrend {
    pub fn new(value: impl Into<String>, change: impl Into<String>, is_positive: bool) -> Self {
        Self {
            value: value.into(),
            change: change.into(),
            is_positive,
            label: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for MetricTrend {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let success = hsla(0.38, 0.7, 0.45, 1.0);
        let error = hsla(0.0, 0.7, 0.5, 1.0);

        let trend_color = if self.is_positive { success } else { error };
        let trend_icon = if self.is_positive { "↑" } else { "↓" };

        div()
            .flex()
            .flex_col()
            .gap_1()
            .when_some(self.label, |d, label| {
                d.child(div().text_xs().text_color(text_muted).child(label))
            })
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap_2()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(text)
                            .child(self.value),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(2.0))
                            .text_sm()
                            .text_color(trend_color)
                            .child(trend_icon)
                            .child(self.change),
                    ),
            )
    }
}
