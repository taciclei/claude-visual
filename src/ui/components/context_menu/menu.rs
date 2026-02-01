//! Main context menu component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Context menu component
#[derive(Clone, IntoElement)]
pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
    min_width: f32,
}

impl ContextMenu {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            min_width: 180.0,
        }
    }

    pub fn items(mut self, items: Vec<ContextMenuItem>) -> Self {
        self.items = items;
        self
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = width;
        self
    }
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ContextMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.22, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let danger = hsla(0.0, 0.7, 0.5, 1.0);

        div()
            .min_w(px(self.min_width))
            .bg(surface)
            .rounded(px(8.0))
            .border_1()
            .border_color(border)
            .shadow_lg()
            .py_1()
            .children(
                self.items.into_iter().map(|item| {
                    match item.item_type {
                        ContextMenuItemType::Separator => {
                            div()
                                .w_full()
                                .h(px(1.0))
                                .my_1()
                                .bg(border)
                                .into_any_element()
                        }
                        _ => {
                            let is_checkbox = matches!(item.item_type, ContextMenuItemType::Checkbox(_));
                            let is_radio = matches!(item.item_type, ContextMenuItemType::Radio { .. });
                            let is_submenu = matches!(item.item_type, ContextMenuItemType::Submenu);
                            let is_checked = matches!(item.item_type,
                                ContextMenuItemType::Checkbox(true) |
                                ContextMenuItemType::Radio { selected: true, .. }
                            );
                            let item_disabled = item.disabled;
                            let item_danger = item.danger;

                            let text_color = if item_disabled {
                                text_muted
                            } else if item_danger {
                                danger
                            } else {
                                text
                            };

                            let mut row = div()
                                .w_full()
                                .px_3()
                                .py_2()
                                .flex()
                                .items_center()
                                .gap_3();

                            if !item_disabled {
                                row = row
                                    .cursor_pointer()
                                    .hover(|s| s.bg(surface_hover));
                            }

                            // Checkbox/Radio indicator
                            if is_checkbox || is_radio {
                                row = row.child(
                                    div()
                                        .w(px(16.0))
                                        .h(px(16.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_xs()
                                        .text_color(if is_checked { accent } else { hsla(0.0, 0.0, 0.0, 0.0) })
                                        .child(if is_checked {
                                            if is_radio { "●" } else { "✓" }
                                        } else {
                                            ""
                                        })
                                );
                            } else if let Some(icon) = &item.icon {
                                row = row.child(
                                    div()
                                        .w(px(16.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_sm()
                                        .child(icon.clone())
                                );
                            } else {
                                row = row.child(div().w(px(16.0)));
                            }

                            // Label
                            row = row.child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .text_color(text_color)
                                    .child(item.label.clone())
                            );

                            // Shortcut or submenu indicator
                            if is_submenu {
                                row = row.child(
                                    div()
                                        .text_xs()
                                        .text_color(text_muted)
                                        .child("›")
                                );
                            } else if let Some(shortcut) = &item.shortcut {
                                row = row.child(
                                    div()
                                        .text_xs()
                                        .text_color(text_muted)
                                        .child(shortcut.clone())
                                );
                            }

                            row.into_any_element()
                        }
                    }
                })
            )
    }
}
