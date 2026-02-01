//! SimpleStat component

use gpui::*;
use gpui::prelude::*;

/// Simple stat display
#[derive(Clone)]
pub struct SimpleStat {
    pub(crate) label: String,
    pub(crate) value: String,
    pub(crate) icon: Option<String>,
}

impl SimpleStat {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            icon: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl RenderOnce for SimpleStat {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .p_4()
            .rounded(px(8.0))
            .border_1()
            .border_color(border)
            .bg(surface)
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .when_some(self.icon, |d, icon| {
                        d.child(
                            div()
                                .text_base()
                                .child(icon)
                        )
                    })
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_muted)
                            .font_weight(FontWeight::MEDIUM)
                            .child(self.label)
                    )
            )
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(text)
                    .child(self.value)
            )
    }
}
