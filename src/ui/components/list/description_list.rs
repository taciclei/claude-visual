//! Description list component (definition list)

use gpui::*;
use gpui::prelude::*;

use super::types::DescriptionLayout;

/// Description list (definition list)
#[derive(Clone)]
pub struct DescriptionList {
    items: Vec<(String, String)>,
    layout: DescriptionLayout,
}

impl DescriptionList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            layout: DescriptionLayout::default(),
        }
    }

    pub fn item(mut self, term: impl Into<String>, description: impl Into<String>) -> Self {
        self.items.push((term.into(), description.into()));
        self
    }

    pub fn items(mut self, items: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        self.items = items.into_iter().map(|(t, d)| (t.into(), d.into())).collect();
        self
    }

    pub fn layout(mut self, layout: DescriptionLayout) -> Self {
        self.layout = layout;
        self
    }
}

impl Default for DescriptionList {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for DescriptionList {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        match self.layout {
            DescriptionLayout::Horizontal => {
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .children(
                        self.items.into_iter().map(|(term, desc)| {
                            div()
                                .w_full()
                                .flex()
                                .items_start()
                                .gap_4()
                                .child(
                                    div()
                                        .w(px(120.0))
                                        .flex_shrink_0()
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(text_muted)
                                        .child(term)
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .text_sm()
                                        .text_color(text)
                                        .child(desc)
                                )
                        })
                    )
            }
            DescriptionLayout::Vertical => {
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .children(
                        self.items.into_iter().map(|(term, desc)| {
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(text_muted)
                                        .child(term.to_uppercase())
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(text)
                                        .child(desc)
                                )
                        })
                    )
            }
            DescriptionLayout::Grid => {
                div()
                    .w_full()
                    .flex()
                    .flex_wrap()
                    .gap_4()
                    .children(
                        self.items.into_iter().map(|(term, desc)| {
                            div()
                                .w(px(200.0))
                                .p_3()
                                .border_1()
                                .border_color(border)
                                .rounded(px(6.0))
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(text_muted)
                                        .child(term)
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(text)
                                        .child(desc)
                                )
                        })
                    )
            }
        }
    }
}
