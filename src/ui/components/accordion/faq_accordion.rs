//! FAQ-style accordion component

use gpui::*;
use gpui::prelude::*;

use super::types::FaqItem;

/// FAQ accordion component
#[derive(Clone)]
pub struct FaqAccordion {
    items: Vec<FaqItem>,
}

impl FaqAccordion {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn item(mut self, question: impl Into<String>, answer: impl Into<String>) -> Self {
        self.items.push(FaqItem::new(question, answer));
        self
    }
}

impl Default for FaqAccordion {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for FaqAccordion {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.6, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .w_full()
            .flex()
            .flex_col()
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let is_first = index == 0;

                div()
                    .w_full()
                    .py_4()
                    .when(!is_first, |d| d.border_t_1().border_color(border))
                    // Question
                    .child(
                        div()
                            .flex()
                            .items_start()
                            .gap_3()
                            .cursor_pointer()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(text_muted)
                                    .child("Q:")
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(text)
                                    .child(item.question)
                            )
                    )
                    // Answer
                    .child(
                        div()
                            .mt_2()
                            .ml(px(24.0))
                            .text_sm()
                            .text_color(text_muted)
                            .child(item.answer)
                    )
            }))
    }
}
