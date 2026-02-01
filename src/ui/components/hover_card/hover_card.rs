//! Main hover card component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// A hover card that appears when hovering over a trigger
#[derive(IntoElement)]
pub struct HoverCard {
    trigger: Div,
    content: Div,
    position: HoverCardPosition,
    arrow: ArrowPosition,
    open: bool,
    delay: u32,
    background: Option<Hsla>,
    border_color: Option<Hsla>,
    shadow: bool,
    max_width: f32,
}

impl HoverCard {
    pub fn new() -> Self {
        Self {
            trigger: div(),
            content: div(),
            position: HoverCardPosition::Bottom,
            arrow: ArrowPosition::Center,
            open: false,
            delay: 200,
            background: None,
            border_color: None,
            shadow: true,
            max_width: 320.0,
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = div().child(trigger);
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = div().child(content);
        self
    }

    pub fn position(mut self, position: HoverCardPosition) -> Self {
        self.position = position;
        self
    }

    pub fn arrow(mut self, arrow: ArrowPosition) -> Self {
        self.arrow = arrow;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn delay(mut self, delay: u32) -> Self {
        self.delay = delay;
        self
    }

    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border_color(mut self, color: Hsla) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }

    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }
}

impl Default for HoverCard {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for HoverCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let background = self.background.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.15,
            a: 1.0,
        });
        let border_color = self.border_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.25,
            a: 1.0,
        });

        let mut container = div().relative();

        // Trigger element
        container = container.child(self.trigger);

        // Card content (only shown when open)
        if self.open {
            let mut card = div()
                .absolute()

                .max_w(px(self.max_width))
                .bg(background)
                .border_1()
                .border_color(border_color)
                .rounded_lg()
                .p_3();

            if self.shadow {
                card = card.shadow_lg();
            }

            // Position the card
            card = match self.position {
                HoverCardPosition::Top => card.bottom_full().mb_2(),
                HoverCardPosition::Bottom => card.top_full().mt_2(),
                HoverCardPosition::Left => card.right_full().mr_2(),
                HoverCardPosition::Right => card.left_full().ml_2(),
                HoverCardPosition::TopStart => card.bottom_full().mb_2().left_0(),
                HoverCardPosition::TopEnd => card.bottom_full().mb_2().right_0(),
                HoverCardPosition::BottomStart => card.top_full().mt_2().left_0(),
                HoverCardPosition::BottomEnd => card.top_full().mt_2().right_0(),
            };

            card = card.child(self.content);
            container = container.child(card);
        }

        container
    }
}
