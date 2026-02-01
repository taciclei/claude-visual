//! Sortable grid component

use super::types::SortableItem;
use gpui::prelude::*;
use gpui::*;

/// Sortable grid - 2D sortable layout
#[derive(IntoElement)]
pub struct SortableGrid {
    id: ElementId,
    pub items: Vec<SortableItem>,
    pub columns: usize,
    item_size: f32,
    gap: f32,
    dragging_id: Option<SharedString>,
}

impl SortableGrid {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            columns: 4,
            item_size: 80.0,
            gap: 8.0,
            dragging_id: None,
        }
    }

    pub fn items(mut self, items: Vec<SortableItem>) -> Self {
        self.items = items;
        self
    }

    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns;
        self
    }

    pub fn item_size(mut self, size: f32) -> Self {
        self.item_size = size;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn dragging_id(mut self, id: impl Into<SharedString>) -> Self {
        self.dragging_id = Some(id.into());
        self
    }
}

impl RenderOnce for SortableGrid {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_wrap()
            .gap(px(self.gap))
            .children(self.items.iter().map(|item| {
                let is_dragging = self.dragging_id.as_ref() == Some(&item.id);
                let opacity = if is_dragging { 0.5 } else { 1.0 };

                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(4.0))
                    .w(px(self.item_size))
                    .h(px(self.item_size))
                    .bg(hsla(0.0, 0.0, 0.12, 1.0))
                    .border_1()
                    .border_color(if is_dragging {
                        hsla(0.6, 0.7, 0.5, 1.0)
                    } else {
                        hsla(0.0, 0.0, 0.2, 1.0)
                    })
                    .rounded(px(8.0))
                    .opacity(opacity)
                    .cursor_grab()
                    .when_some(item.icon.clone(), |el, icon| {
                        el.child(div().text_size(px(24.0)).child(icon))
                    })
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                            .text_ellipsis()
                            .max_w(px(self.item_size - 8.0))
                            .child(item.content.clone()),
                    )
            }))
    }
}
