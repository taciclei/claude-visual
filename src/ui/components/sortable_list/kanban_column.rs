//! Kanban column component

use gpui::*;
use gpui::prelude::*;
use super::types::SortableItem;

/// Kanban column for sortable cards
#[derive(IntoElement)]
pub struct KanbanColumn {
    id: ElementId,
    title: SharedString,
    pub items: Vec<SortableItem>,
    is_drop_target: bool,
    color: Option<gpui::Hsla>,
    pub max_items: Option<usize>,
}

impl KanbanColumn {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            items: Vec::new(),
            is_drop_target: false,
            color: None,
            max_items: None,
        }
    }

    pub fn items(mut self, items: Vec<SortableItem>) -> Self {
        self.items = items;
        self
    }

    pub fn is_drop_target(mut self, is_target: bool) -> Self {
        self.is_drop_target = is_target;
        self
    }

    pub fn color(mut self, color: gpui::Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn max_items(mut self, max: usize) -> Self {
        self.max_items = Some(max);
        self
    }
}

impl RenderOnce for KanbanColumn {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let header_color = self.color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0));
        let is_over_limit = self.max_items.map(|m| self.items.len() > m).unwrap_or(false);

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .w(px(280.0))
            .min_h(px(200.0))
            .bg(hsla(0.0, 0.0, 0.08, 1.0))
            .border_1()
            .border_color(if self.is_drop_target {
                hsla(0.6, 0.7, 0.5, 1.0)
            } else {
                hsla(0.0, 0.0, 0.15, 1.0)
            })
            .rounded(px(8.0))
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(12.0))
                    .py(px(10.0))
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.15, 1.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .w(px(4.0))
                                    .h(px(16.0))
                                    .rounded(px(2.0))
                                    .bg(header_color)
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                    .child(self.title.clone())
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(2.0))
                                    .bg(if is_over_limit {
                                        hsla(0.0, 0.7, 0.5, 0.2)
                                    } else {
                                        hsla(0.0, 0.0, 0.15, 1.0)
                                    })
                                    .rounded(px(10.0))
                                    .text_size(px(12.0))
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .text_color(if is_over_limit {
                                        hsla(0.0, 0.7, 0.6, 1.0)
                                    } else {
                                        hsla(0.0, 0.0, 0.6, 1.0)
                                    })
                                    .child(format!(
                                        "{}{}",
                                        self.items.len(),
                                        self.max_items.map(|m| format!("/{}", m)).unwrap_or_default()
                                    ))
                            )
                    )
            )
            // Items
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(8.0))
                    .flex_1()
                    .id("scroll-kanban-items")
                    .overflow_y_scroll()
                    .when(self.items.is_empty(), |el| {
                        el.child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .h(px(60.0))
                                .border_2()
                                .border_dashed()
                                .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                                .rounded(px(6.0))
                                .text_size(px(12.0))
                                .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                                .child("Drop items here")
                        )
                    })
                    .children(self.items.iter().map(|item| {
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .p(px(10.0))
                            .bg(hsla(0.0, 0.0, 0.12, 1.0))
                            .border_1()
                            .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                            .rounded(px(6.0))
                            .shadow_sm()
                            .cursor_grab()
                            .when_some(item.icon.clone(), |el, icon| {
                                el.child(
                                    div()
                                        .text_size(px(14.0))
                                        .child(icon)
                                )
                            })
                            .child(
                                div()
                                    .flex_1()
                                    .text_size(px(13.0))
                                    .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                    .child(item.content.clone())
                            )
                    }))
            )
    }
}
