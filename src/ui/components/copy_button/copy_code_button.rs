//! Copy code block button component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Copy code block button - designed for code blocks
#[derive(IntoElement)]
pub struct CopyCodeButton {
    pub(crate) id: ElementId,
    pub(crate) code: SharedString,
    pub(crate) state: CopyState,
    pub(crate) position: CopyCodePosition,
}

impl CopyCodeButton {
    pub fn new(id: impl Into<ElementId>, code: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            code: code.into(),
            state: CopyState::default(),
            position: CopyCodePosition::default(),
        }
    }

    pub fn state(mut self, state: CopyState) -> Self {
        self.state = state;
        self
    }

    pub fn position(mut self, position: CopyCodePosition) -> Self {
        self.position = position;
        self
    }
}

impl RenderOnce for CopyCodeButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (icon, text, color) = match self.state {
            CopyState::Idle => ("📋", "Copy", hsla(0.0, 0.0, 0.6, 1.0)),
            CopyState::Copying => ("⏳", "Copying", hsla(0.0, 0.0, 0.5, 1.0)),
            CopyState::Copied => ("✓", "Copied", hsla(0.35, 0.7, 0.45, 1.0)),
            CopyState::Error => ("✗", "Error", hsla(0.0, 0.7, 0.5, 1.0)),
        };

        div()
            .id(self.id)
            .absolute()
            .when(matches!(self.position, CopyCodePosition::TopRight | CopyCodePosition::BottomRight), |el| {
                el.right(px(8.0))
            })
            .when(matches!(self.position, CopyCodePosition::TopLeft | CopyCodePosition::BottomLeft), |el| {
                el.left(px(8.0))
            })
            .when(matches!(self.position, CopyCodePosition::TopRight | CopyCodePosition::TopLeft), |el| {
                el.top(px(8.0))
            })
            .when(matches!(self.position, CopyCodePosition::BottomRight | CopyCodePosition::BottomLeft), |el| {
                el.bottom(px(8.0))
            })
            .px(px(8.0))
            .py(px(4.0))
            .flex()
            .items_center()
            .gap(px(4.0))
            .rounded(px(4.0))
            .bg(hsla(0.0, 0.0, 0.0, 0.6))
            .cursor_pointer()
            .hover(|style| style.bg(hsla(0.0, 0.0, 0.0, 0.8)))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(color)
                    .child(icon)
            )
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(color)
                    .child(text)
            )
    }
}
