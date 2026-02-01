//! Basic marquee component - scrolling content

use gpui::prelude::*;
use gpui::*;

use super::types::*;

/// Marquee component - scrolling content
#[derive(IntoElement)]
pub struct Marquee {
    pub(crate) id: ElementId,
    pub(crate) children: Vec<gpui::AnyElement>,
    pub(crate) direction: MarqueeDirection,
    pub(crate) speed: MarqueeSpeed,
    pub(crate) pause_on_hover: bool,
    pub(crate) gap: f32,
    pub(crate) gradient: bool,
    pub(crate) gradient_width: f32,
}

impl Marquee {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            direction: MarqueeDirection::default(),
            speed: MarqueeSpeed::default(),
            pause_on_hover: true,
            gap: 40.0,
            gradient: true,
            gradient_width: 40.0,
        }
    }

    pub fn children(mut self, children: Vec<impl IntoElement>) -> Self {
        self.children = children.into_iter().map(|c| c.into_any_element()).collect();
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn direction(mut self, direction: MarqueeDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn speed(mut self, speed: MarqueeSpeed) -> Self {
        self.speed = speed;
        self
    }

    pub fn pause_on_hover(mut self, pause: bool) -> Self {
        self.pause_on_hover = pause;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn gradient(mut self, gradient: bool) -> Self {
        self.gradient = gradient;
        self
    }

    pub fn gradient_width(mut self, width: f32) -> Self {
        self.gradient_width = width;
        self
    }
}

impl RenderOnce for Marquee {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let is_vertical = matches!(self.direction, MarqueeDirection::Up | MarqueeDirection::Down);

        div()
            .id(self.id)
            .w_full()
            .overflow_hidden()
            .relative()
            .when(is_vertical, |el| el.h(px(200.0)))
            .when(!is_vertical, |el| el.h_auto())
            .child(
                // Content container
                div()
                    .flex()
                    .when(is_vertical, |el| el.flex_col())
                    .when(!is_vertical, |el| el.flex_row())
                    .gap(px(self.gap))
                    .children(self.children)
            )
    }
}
