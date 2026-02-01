//! News ticker - scrolling news headlines

use gpui::prelude::*;
use gpui::*;

use super::types::*;

/// News ticker - scrolling news headlines
#[derive(IntoElement)]
pub struct NewsTicker {
    id: ElementId,
    headlines: Vec<SharedString>,
    label: SharedString,
    speed: MarqueeSpeed,
    label_color: Option<gpui::Hsla>,
}

impl NewsTicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            headlines: Vec::new(),
            label: "BREAKING".into(),
            speed: MarqueeSpeed::default(),
            label_color: None,
        }
    }

    pub fn headlines(mut self, headlines: Vec<impl Into<SharedString>>) -> Self {
        self.headlines = headlines.into_iter().map(|h| h.into()).collect();
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn speed(mut self, speed: MarqueeSpeed) -> Self {
        self.speed = speed;
        self
    }

    pub fn label_color(mut self, color: gpui::Hsla) -> Self {
        self.label_color = Some(color);
        self
    }
}

impl RenderOnce for NewsTicker {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let label_color = self.label_color.unwrap_or(hsla(0.0, 0.8, 0.5, 1.0));

        div()
            .id(self.id)
            .w_full()
            .h(px(36.0))
            .bg(hsla(0.0, 0.0, 0.05, 1.0))
            .flex()
            .items_center()
            .child(
                // Label
                div()
                    .px(px(12.0))
                    .h_full()
                    .flex()
                    .items_center()
                    .bg(label_color)
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                            .child(self.label.clone())
                    )
            )
            .child(
                // Scrolling content
                div()
                    .flex_1()
                    .h_full()
                    .overflow_hidden()
                    .flex()
                    .items_center()
                    .px(px(16.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(32.0))
                            .children(self.headlines.into_iter().map(|headline| {
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(32.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(hsla(0.0, 0.0, 0.8, 1.0))
                                            .whitespace_nowrap()
                                            .child(headline)
                                    )
                                    .child(
                                        div()
                                            .text_size(px(8.0))
                                            .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                                            .child("•")
                                    )
                            }))
                    )
            )
    }
}
