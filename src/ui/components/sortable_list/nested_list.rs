//! Nested sortable list component

use gpui::*;
use gpui::prelude::*;
use super::types::NestedSortableItem;

/// Nested sortable list component
#[derive(IntoElement)]
pub struct NestedSortableList {
    id: ElementId,
    pub items: Vec<NestedSortableItem>,
    indent_size: f32,
}

impl NestedSortableList {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            indent_size: 24.0,
        }
    }

    pub fn items(mut self, items: Vec<NestedSortableItem>) -> Self {
        self.items = items;
        self
    }

    pub fn indent_size(mut self, size: f32) -> Self {
        self.indent_size = size;
        self
    }

    fn render_item(item: &NestedSortableItem, depth: usize, indent_size: f32) -> gpui::AnyElement {
        let indent = depth as f32 * indent_size;
        let has_children = !item.children.is_empty();

        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .pl(px(indent))
                    .pr(px(12.0))
                    .py(px(8.0))
                    .cursor_grab()
                    // Expand/collapse
                    .when(has_children, |el| {
                        el.child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(20.0))
                                .h(px(20.0))
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .text_size(px(12.0))
                                .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                .child(if item.collapsed { "▶" } else { "▼" })
                        )
                    })
                    .when(!has_children, |el| {
                        el.child(div().w(px(20.0)))
                    })
                    // Drag handle
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                            .cursor_grab()
                            .child("⋮⋮")
                    )
                    // Content
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(14.0))
                            .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                            .child(item.content.clone())
                    )
            )
            .when(!item.collapsed && has_children, |el| {
                el.children(item.children.iter().map(|child| {
                    Self::render_item(child, depth + 1, indent_size)
                }))
            })
            .into_any_element()
    }
}

impl RenderOnce for NestedSortableList {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let indent_size = self.indent_size;

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .children(self.items.iter().map(|item| {
                Self::render_item(item, 0, indent_size)
            }))
    }
}
