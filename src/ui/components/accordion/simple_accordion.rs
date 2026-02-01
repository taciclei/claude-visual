//! Simple accordion for stateless rendering

use gpui::*;
use gpui::prelude::*;

use super::types::SimpleAccordionItem;

/// Simple accordion for stateless rendering
#[derive(Clone)]
pub struct SimpleAccordion {
    items: Vec<SimpleAccordionItem>,
    default_open: Option<usize>,
}

impl SimpleAccordion {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            default_open: None,
        }
    }

    pub fn item(mut self, item: SimpleAccordionItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn default_open(mut self, index: usize) -> Self {
        self.default_open = Some(index);
        self
    }
}

impl Default for SimpleAccordion {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for SimpleAccordion {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.6, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .w_full()
            .rounded(px(8.0))
            .border_1()
            .border_color(border)
            .overflow_hidden()
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let is_open = self.default_open == Some(index);
                let is_first = index == 0;
                let chevron = if is_open { "▼" } else { "▶" };

                div()
                    .w_full()
                    .when(!is_first, |d| d.border_t_1().border_color(border))
                    // Header
                    .child(
                        div()
                            .h(px(44.0))
                            .px_4()
                            .flex()
                            .items_center()
                            .gap_2()
                            .bg(surface)
                            .cursor_pointer()
                            .hover(|s| s.bg(surface_hover))
                            .when_some(item.icon, |d, icon| {
                                d.child(
                                    div()
                                        .text_sm()
                                        .child(icon)
                                )
                            })
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(text)
                                    .child(item.title)
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .child(chevron)
                            )
                    )
                    // Content
                    .when(is_open, |d| {
                        d.child(
                            div()
                                .px_4()
                                .py_3()
                                .border_t_1()
                                .border_color(border)
                                .text_sm()
                                .text_color(text_muted)
                                .child(item.content)
                        )
                    })
            }))
    }
}
