//! Action list component for menus and command palettes

use gpui::*;
use gpui::prelude::*;

use super::types::{ActionItem, ListSize};

/// Action list (for menus, command palettes)
#[derive(Clone)]
pub struct ActionList {
    items: Vec<ActionItem>,
    size: ListSize,
}

impl ActionList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            size: ListSize::default(),
        }
    }

    pub fn items(mut self, items: Vec<ActionItem>) -> Self {
        self.items = items;
        self
    }

    pub fn size(mut self, size: ListSize) -> Self {
        self.size = size;
        self
    }
}

impl Default for ActionList {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ActionList {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let danger = hsla(0.0, 0.7, 0.5, 1.0);

        let (px_h, py_v) = self.size.item_padding();

        div()
            .w_full()
            .flex()
            .flex_col()
            .children(
                self.items.into_iter().map(move |item| {
                    let is_disabled = item.disabled;
                    let is_danger = item.danger;
                    let text_color = if is_danger { danger } else { text };

                    let mut row = div()
                        .w_full()
                        .px(px(px_h))
                        .py(px(py_v))
                        .flex()
                        .items_center()
                        .gap_3()
                        .rounded(px(4.0));

                    if !is_disabled {
                        row = row
                            .cursor_pointer()
                            .hover(|s| s.bg(surface_hover));
                    } else {
                        row = row.opacity(0.5);
                    }

                    // Icon
                    if let Some(icon) = item.icon {
                        row = row.child(
                            div()
                                .w(px(20.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(text_muted)
                                .child(icon)
                        );
                    }

                    // Label and description
                    row = row.child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(text_color)
                                    .child(item.label)
                            )
                            .when_some(item.description, |d, desc| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(text_muted)
                                        .child(desc)
                                )
                            })
                    );

                    // Shortcut
                    if let Some(shortcut) = item.shortcut {
                        row = row.child(
                            div()
                                .text_xs()
                                .text_color(text_muted)
                                .font_family("monospace")
                                .child(shortcut)
                        );
                    }

                    row
                })
            )
    }
}
