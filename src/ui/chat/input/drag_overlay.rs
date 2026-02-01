//! Drag-and-drop overlay rendering

use gpui::prelude::*;
use gpui::*;

use super::ChatInput;

impl ChatInput {
    /// Render drag-and-drop overlay when files are being dragged
    pub(super) fn render_drag_overlay(&self) -> impl IntoElement {
        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(210.0 / 360.0, 0.5, 0.2, 0.95))
            .rounded_lg()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(div().text_xl().child("📎"))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                            .child("Drop files to attach"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(hsla(0.0, 0.0, 0.8, 1.0))
                            .child("Files will be added as @file: mentions"),
                    ),
            )
    }
}
