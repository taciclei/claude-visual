//! Translation status indicator component

use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;

/// Translation status indicator
#[derive(IntoElement)]
pub struct TranslationStatus {
    id: ElementId,
    total_strings: usize,
    translated_strings: usize,
    language: SharedString,
}

impl TranslationStatus {
    pub fn new(id: impl Into<ElementId>, language: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            total_strings: 100,
            translated_strings: 0,
            language: language.into(),
        }
    }

    pub fn total(mut self, total: usize) -> Self {
        self.total_strings = total;
        self
    }

    pub fn translated(mut self, translated: usize) -> Self {
        self.translated_strings = translated;
        self
    }
}

impl RenderOnce for TranslationStatus {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let percentage = if self.total_strings > 0 {
            (self.translated_strings as f32 / self.total_strings as f32 * 100.0) as i32
        } else {
            0
        };

        let color = if percentage >= 90 {
            hsla(0.35, 0.7, 0.45, 1.0)
        } else if percentage >= 50 {
            hsla(0.12, 0.9, 0.5, 1.0)
        } else {
            hsla(0.0, 0.7, 0.5, 1.0)
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                    .child(self.language.clone()),
            )
            .child(
                div()
                    .flex_1()
                    .h(px(6.0))
                    .bg(hsla(0.0, 0.0, 0.2, 1.0))
                    .rounded_full()
                    .child(
                        div()
                            .h_full()
                            .w(pct(percentage as f32))
                            .bg(color)
                            .rounded_full(),
                    ),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(color)
                    .child(format!("{}%", percentage)),
            )
    }
}
