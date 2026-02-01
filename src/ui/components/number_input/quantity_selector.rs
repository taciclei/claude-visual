//! Quantity selector with presets

use gpui::prelude::*;
use gpui::*;

/// Quantity selector with presets
#[derive(Clone)]
pub struct QuantitySelector {
    value: u32,
    presets: Vec<u32>,
}

impl QuantitySelector {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            presets: Vec::new(),
        }
    }

    pub fn presets(mut self, presets: Vec<u32>) -> Self {
        self.presets = presets;
        self
    }
}

impl RenderOnce for QuantitySelector {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.2, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        div()
            .flex()
            .flex_col()
            .gap_2()
            // Stepper
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    // Decrement
                    .child(
                        div()
                            .size(px(32.0))
                            .rounded_full()
                            .border_1()
                            .border_color(border)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .text_color(text)
                            .cursor_pointer()
                            .hover(|s| s.bg(surface_hover))
                            .child("−"),
                    )
                    // Value
                    .child(
                        div()
                            .min_w(px(48.0))
                            .text_xl()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text)
                            .text_center()
                            .child(self.value.to_string()),
                    )
                    // Increment
                    .child(
                        div()
                            .size(px(32.0))
                            .rounded_full()
                            .border_1()
                            .border_color(border)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .text_color(text)
                            .cursor_pointer()
                            .hover(|s| s.bg(surface_hover))
                            .child("+"),
                    ),
            )
            // Presets
            .when(!self.presets.is_empty(), |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .children(self.presets.into_iter().map(|preset| {
                            let is_selected = preset == self.value;
                            div()
                                .px_3()
                                .py_1()
                                .rounded(px(4.0))
                                .text_sm()
                                .when(is_selected, |d| d.bg(accent).text_color(white()))
                                .when(!is_selected, |d| {
                                    d.bg(surface)
                                        .text_color(text_muted)
                                        .cursor_pointer()
                                        .hover(|s| s.bg(surface_hover))
                                })
                                .child(preset.to_string())
                        })),
                )
            })
    }
}
