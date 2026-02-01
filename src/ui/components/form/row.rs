//! Form row - horizontal grouping of fields

use gpui::prelude::*;
use gpui::*;

#[derive(IntoElement)]
pub struct FormRow {
    children: Vec<gpui::AnyElement>,
    gap: f32,
}

impl FormRow {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            gap: 16.0,
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

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
}

impl Default for FormRow {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for FormRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().w_full().flex().flex_row().gap(px(self.gap)).children(
            self.children
                .into_iter()
                .map(|child| div().flex_1().child(child)),
        )
    }
}
