//! ProgressStat component

use gpui::*;
use gpui::prelude::*;

/// Progress stat with bar
#[derive(Clone)]
pub struct ProgressStat {
    label: String,
    value: f32,
    pub(crate) max: f32,
    format: String,
}

impl ProgressStat {
    pub fn new(label: impl Into<String>, value: f32, max: f32) -> Self {
        Self {
            label: label.into(),
            value,
            max,
            format: "{value}/{max}".to_string(),
        }
    }

    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = format.into();
        self
    }

    pub fn percentage(label: impl Into<String>, percent: f32) -> Self {
        Self {
            label: label.into(),
            value: percent,
            max: 100.0,
            format: "{value}%".to_string(),
        }
    }
}

impl RenderOnce for ProgressStat {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let surface = hsla(0.0, 0.0, 0.2, 1.0);

        let progress = (self.value / self.max).clamp(0.0, 1.0);
        let value_str = self.format
            .replace("{value}", &format!("{:.0}", self.value))
            .replace("{max}", &format!("{:.0}", self.max));

        div()
            .flex()
            .flex_col()
            .gap_2()
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_muted)
                            .child(self.label)
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text)
                            .child(value_str)
                    )
            )
            // Progress bar
            .child(
                div()
                    .w_full()
                    .h(px(8.0))
                    .rounded_full()
                    .bg(surface)
                    .child(
                        div()
                            .h_full()
                            .w(relative(progress))
                            .rounded_full()
                            .bg(accent)
                    )
            )
    }
}
