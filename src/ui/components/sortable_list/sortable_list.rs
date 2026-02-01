//! Sortable list component

use super::types::{SortableItem, SortableVariant};
use gpui::prelude::*;
use gpui::*;

/// Sortable list component
#[derive(IntoElement)]
pub struct SortableList {
    id: ElementId,
    pub items: Vec<SortableItem>,
    dragging_id: Option<SharedString>,
    drop_target_id: Option<SharedString>,
    pub variant: SortableVariant,
    pub show_handle: bool,
    pub show_numbers: bool,
    disabled: bool,
}

impl SortableList {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            dragging_id: None,
            drop_target_id: None,
            variant: SortableVariant::default(),
            show_handle: true,
            show_numbers: false,
            disabled: false,
        }
    }

    pub fn items(mut self, items: Vec<SortableItem>) -> Self {
        self.items = items;
        self
    }

    pub fn dragging_id(mut self, id: impl Into<SharedString>) -> Self {
        self.dragging_id = Some(id.into());
        self
    }

    pub fn drop_target_id(mut self, id: impl Into<SharedString>) -> Self {
        self.drop_target_id = Some(id.into());
        self
    }

    pub fn variant(mut self, variant: SortableVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn show_handle(mut self, show: bool) -> Self {
        self.show_handle = show;
        self
    }

    pub fn show_numbers(mut self, show: bool) -> Self {
        self.show_numbers = show;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for SortableList {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let gap = match self.variant {
            SortableVariant::Default => 4.0,
            SortableVariant::Cards => 8.0,
            SortableVariant::Compact => 0.0,
            SortableVariant::Bordered => 0.0,
        };

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(gap))
            .when(self.variant == SortableVariant::Bordered, |el| {
                el.border_1()
                    .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                    .rounded(px(8.0))
                    .overflow_hidden()
            })
            .children(self.items.iter().enumerate().map(|(i, item)| {
                let is_dragging = self.dragging_id.as_ref() == Some(&item.id);
                let is_drop_target = self.drop_target_id.as_ref() == Some(&item.id);
                let is_disabled = self.disabled || item.disabled;
                let is_locked = item.locked;

                let (bg_color, border_color) = match self.variant {
                    SortableVariant::Default => {
                        if is_dragging {
                            (hsla(0.6, 0.5, 0.4, 0.3), hsla(0.6, 0.7, 0.5, 1.0))
                        } else if is_drop_target {
                            (hsla(0.6, 0.5, 0.4, 0.15), hsla(0.6, 0.7, 0.5, 0.5))
                        } else {
                            (hsla(0.0, 0.0, 0.12, 1.0), hsla(0.0, 0.0, 0.2, 1.0))
                        }
                    }
                    SortableVariant::Cards => {
                        if is_dragging {
                            (hsla(0.6, 0.5, 0.4, 0.2), hsla(0.6, 0.7, 0.5, 1.0))
                        } else {
                            (hsla(0.0, 0.0, 0.1, 1.0), hsla(0.0, 0.0, 0.2, 1.0))
                        }
                    }
                    SortableVariant::Compact | SortableVariant::Bordered => {
                        if is_drop_target {
                            (hsla(0.6, 0.5, 0.4, 0.1), hsla(0.0, 0.0, 0.0, 0.0))
                        } else {
                            (hsla(0.0, 0.0, 0.0, 0.0), hsla(0.0, 0.0, 0.0, 0.0))
                        }
                    }
                };

                let opacity = if is_disabled {
                    0.5
                } else if is_dragging {
                    0.8
                } else {
                    1.0
                };

                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .px(px(12.0))
                    .py(px(10.0))
                    .bg(bg_color)
                    .when(self.variant == SortableVariant::Cards, |el| {
                        el.border_1()
                            .border_color(border_color)
                            .rounded(px(8.0))
                            .shadow_sm()
                    })
                    .when(self.variant == SortableVariant::Bordered && i > 0, |el| {
                        el.border_t_1().border_color(hsla(0.0, 0.0, 0.2, 1.0))
                    })
                    .when(is_drop_target, |el| {
                        el.border_t_2().border_color(hsla(0.6, 0.7, 0.5, 1.0))
                    })
                    .opacity(opacity)
                    .when(!is_disabled && !is_locked, |el| el.cursor_grab())
                    // Drag handle
                    .when(self.show_handle && !is_locked, |el| {
                        el.child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(20.0))
                                .text_size(px(14.0))
                                .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                                .when(!is_disabled, |el| el.cursor_grab())
                                .child("⋮⋮"),
                        )
                    })
                    .when(is_locked, |el| {
                        el.child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(20.0))
                                .text_size(px(12.0))
                                .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                                .child("🔒"),
                        )
                    })
                    // Number
                    .when(self.show_numbers, |el| {
                        el.child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(24.0))
                                .h(px(24.0))
                                .rounded_full()
                                .bg(hsla(0.0, 0.0, 0.2, 1.0))
                                .text_size(px(12.0))
                                .font_weight(gpui::FontWeight::MEDIUM)
                                .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                                .child((i + 1).to_string()),
                        )
                    })
                    // Icon
                    .when_some(item.icon.clone(), |el, icon| {
                        el.child(div().text_size(px(16.0)).child(icon))
                    })
                    // Content
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(14.0))
                            .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                            .child(item.content.clone()),
                    )
            }))
    }
}
