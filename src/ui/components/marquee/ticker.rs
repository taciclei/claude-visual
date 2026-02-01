//! Ticker - stock ticker style scrolling

use gpui::prelude::*;
use gpui::*;

use super::types::*;

/// Ticker - stock ticker style scrolling
#[derive(IntoElement)]
pub struct Ticker {
    id: ElementId,
    items: Vec<TickerItem>,
    speed: MarqueeSpeed,
    separator: SharedString,
}

impl Ticker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            speed: MarqueeSpeed::default(),
            separator: "•".into(),
        }
    }

    pub fn items(mut self, items: Vec<TickerItem>) -> Self {
        self.items = items;
        self
    }

    pub fn speed(mut self, speed: MarqueeSpeed) -> Self {
        self.speed = speed;
        self
    }

    pub fn separator(mut self, separator: impl Into<SharedString>) -> Self {
        self.separator = separator.into();
        self
    }
}

impl RenderOnce for Ticker {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .w_full()
            .h(px(32.0))
            .bg(hsla(0.0, 0.0, 0.08, 1.0))
            .overflow_hidden()
            .child(
                div()
                    .h_full()
                    .flex()
                    .items_center()
                    .gap(px(24.0))
                    .px(px(16.0))
                    .children(self.items.into_iter().map(|item| {
                        let change_color = item.change.map(|c| {
                            if c >= 0.0 {
                                hsla(0.35, 0.7, 0.45, 1.0)
                            } else {
                                hsla(0.0, 0.7, 0.5, 1.0)
                            }
                        });

                        let change_text = item.change.map(|c| {
                            if c >= 0.0 {
                                format!("+{:.2}%", c)
                            } else {
                                format!("{:.2}%", c)
                            }
                        });

                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                                    .child(item.symbol.clone())
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                    .child(item.value.clone())
                            )
                            .when(change_text.is_some(), |el| {
                                el.child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(change_color.unwrap_or(hsla(0.0, 0.0, 0.6, 1.0)))
                                        .child(change_text.unwrap_or_default())
                                )
                            })
                    }))
            )
    }
}
