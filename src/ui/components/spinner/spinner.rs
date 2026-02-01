//! Basic loading spinner component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Basic loading spinner
#[derive(Clone, IntoElement)]
pub struct Spinner {
    pub(crate) size: SpinnerSize,
    pub(crate) variant: SpinnerVariant,
    pub(crate) color: Option<Hsla>,
    pub(crate) label: Option<String>,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            size: SpinnerSize::default(),
            variant: SpinnerVariant::default(),
            color: None,
            label: None,
        }
    }

    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: SpinnerVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Spinner {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        let spinner_color = self.color.unwrap_or(accent);
        let size = self.size.size();

        let spinner_element = match self.variant {
            SpinnerVariant::Circular | SpinnerVariant::Ring => {
                // Circular spinner representation
                div()
                    .size(px(size))
                    .rounded_full()
                    .border_2()
                    .border_color(spinner_color.opacity(0.2))
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .rounded_full()
                            .border_2()
                            .border_color(hsla(0.0, 0.0, 0.0, 0.0))
                            .border_t_2()
                            .border_color(spinner_color),
                    )
                    .into_any_element()
            }
            SpinnerVariant::Dots => {
                // Three dots
                let dot_size = size / 4.0;
                div()
                    .flex()
                    .items_center()
                    .gap(px(dot_size / 2.0))
                    .child(div().size(px(dot_size)).rounded_full().bg(spinner_color))
                    .child(
                        div()
                            .size(px(dot_size))
                            .rounded_full()
                            .bg(spinner_color.opacity(0.6)),
                    )
                    .child(
                        div()
                            .size(px(dot_size))
                            .rounded_full()
                            .bg(spinner_color.opacity(0.3)),
                    )
                    .into_any_element()
            }
            SpinnerVariant::Bars => {
                // Three vertical bars
                let bar_width = size / 6.0;
                let bar_height = size;
                div()
                    .h(px(bar_height))
                    .flex()
                    .items_end()
                    .gap(px(bar_width / 2.0))
                    .child(
                        div()
                            .w(px(bar_width))
                            .h(px(bar_height * 0.6))
                            .rounded(px(bar_width / 2.0))
                            .bg(spinner_color),
                    )
                    .child(
                        div()
                            .w(px(bar_width))
                            .h(px(bar_height))
                            .rounded(px(bar_width / 2.0))
                            .bg(spinner_color.opacity(0.7)),
                    )
                    .child(
                        div()
                            .w(px(bar_width))
                            .h(px(bar_height * 0.4))
                            .rounded(px(bar_width / 2.0))
                            .bg(spinner_color.opacity(0.4)),
                    )
                    .into_any_element()
            }
        };

        let mut container = div().flex().items_center().gap_2();

        container = container.child(spinner_element);

        if let Some(label_text) = self.label {
            container = container.child(div().text_sm().text_color(text_muted).child(label_text));
        }

        container
    }
}
