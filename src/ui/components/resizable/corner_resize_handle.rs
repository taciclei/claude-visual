//! Corner resize handle component

use super::types::CornerPosition;
use gpui::prelude::*;
use gpui::*;

/// Corner resize handle for windows/dialogs
#[derive(IntoElement)]
pub struct CornerResizeHandle {
    pub(crate) position: CornerPosition,
    pub(crate) size: f32,
    color: Option<Hsla>,
    pub(crate) visible: bool,
}

impl CornerResizeHandle {
    pub fn new() -> Self {
        Self {
            position: CornerPosition::BottomRight,
            size: 16.0,
            color: None,
            visible: true,
        }
    }

    pub fn position(mut self, position: CornerPosition) -> Self {
        self.position = position;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

impl Default for CornerResizeHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for CornerResizeHandle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if !self.visible {
            return div();
        }

        let color = self.color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.4,
            a: 1.0,
        });

        let cursor = match self.position {
            CornerPosition::TopLeft | CornerPosition::BottomRight => {
                CursorStyle::ResizeUpLeftDownRight
            }
            CornerPosition::TopRight | CornerPosition::BottomLeft => {
                CursorStyle::ResizeUpRightDownLeft
            }
        };

        // Position the handle in the corner
        let mut handle = div()
            .absolute()
            .w(px(self.size))
            .h(px(self.size))
            .cursor(cursor);

        handle = match self.position {
            CornerPosition::TopLeft => handle.top_0().left_0(),
            CornerPosition::TopRight => handle.top_0().right_0(),
            CornerPosition::BottomLeft => handle.bottom_0().left_0(),
            CornerPosition::BottomRight => handle.bottom_0().right_0(),
        };

        // Diagonal lines pattern for the grip
        let line_positions = match self.position {
            CornerPosition::BottomRight => vec![
                (self.size - 4.0, self.size - 8.0),
                (self.size - 8.0, self.size - 4.0),
            ],
            _ => vec![],
        };

        for (x, y) in line_positions {
            handle = handle.child(
                div()
                    .absolute()
                    .left(px(x))
                    .top(px(y))
                    .w(px(2.0))
                    .h(px(2.0))
                    .rounded_full()
                    .bg(color),
            );
        }

        handle
    }
}
