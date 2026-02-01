//! Positioned FAB container component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Positioned FAB container
#[derive(IntoElement)]
pub struct FabContainer {
    id: ElementId,
    position: FabPosition,
    offset: f32,
    children: Vec<gpui::AnyElement>,
}

impl FabContainer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            position: FabPosition::default(),
            offset: 16.0,
            children: Vec::new(),
        }
    }

    pub fn position(mut self, position: FabPosition) -> Self {
        self.position = position;
        self
    }

    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl RenderOnce for FabContainer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let offset = self.offset;

        div()
            .id(self.id)
            .absolute()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .map(|el| match self.position {
                FabPosition::BottomRight => el.bottom(px(offset)).right(px(offset)),
                FabPosition::BottomLeft => el.bottom(px(offset)).left(px(offset)),
                FabPosition::BottomCenter => el.bottom(px(offset)).left_1_2(),
                FabPosition::TopRight => el.top(px(offset)).right(px(offset)),
                FabPosition::TopLeft => el.top(px(offset)).left(px(offset)),
            })
            .children(self.children)
    }
}
