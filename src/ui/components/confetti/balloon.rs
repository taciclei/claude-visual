use gpui::*;
use gpui::prelude::*;
use super::types::BalloonSize;

/// Balloon floating animation
#[derive(IntoElement)]
pub struct Balloon {
    id: ElementId,
    color: gpui::Hsla,
    size: BalloonSize,
    x: f32,
    y: f32,
    floating: bool,
    string_visible: bool,
}

impl Balloon {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            color: rgb(0xef4444).into(),
            size: BalloonSize::default(),
            x: 0.0,
            y: 0.0,
            floating: true,
            string_visible: true,
        }
    }

    pub fn color(mut self, color: gpui::Hsla) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: BalloonSize) -> Self {
        self.size = size;
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn floating(mut self, floating: bool) -> Self {
        self.floating = floating;
        self
    }

    pub fn string_visible(mut self, visible: bool) -> Self {
        self.string_visible = visible;
        self
    }
}

impl RenderOnce for Balloon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (width, height) = self.size.dimensions();

        div()
            .id(self.id)
            .absolute()
            .left(px(self.x))
            .top(px(self.y))
            .flex()
            .flex_col()
            .items_center()
            .child(
                // Balloon body (oval shape approximation)
                div()
                    .w(px(width))
                    .h(px(height))
                    .rounded_full()
                    .bg(self.color)
                    .relative()
                    .child(
                        // Highlight
                        div()
                            .absolute()
                            .left(px(width * 0.2))
                            .top(px(height * 0.15))
                            .w(px(width * 0.25))
                            .h(px(height * 0.2))
                            .rounded_full()
                            .bg(rgba(0xffffff66)),
                    ),
            )
            .when(self.string_visible, |d| {
                d.child(
                    // Balloon knot and string
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .child(
                            // Knot
                            div()
                                .w(px(8.0))
                                .h(px(6.0))
                                .bg(self.color.opacity(0.8)),
                        )
                        .child(
                            // String
                            div()
                                .w(px(1.0))
                                .h(px(40.0))
                                .bg(rgba(0x666666ff)),
                        ),
                )
            })
    }
}
