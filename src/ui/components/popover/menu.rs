//! Menu popover for action lists

use gpui::*;
use gpui::prelude::*;

/// Menu popover for action lists
#[derive(Clone)]
pub struct MenuPopover {
    items: Vec<MenuPopoverItem>,
    width: f32,
}

#[derive(Clone)]
pub struct MenuPopoverItem {
    pub(crate) label: String,
    pub(crate) icon: Option<String>,
    pub(crate) shortcut: Option<String>,
    pub(crate) disabled: bool,
    pub(crate) danger: bool,
}

impl MenuPopoverItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            danger: false,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }
}

impl MenuPopover {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            width: 180.0,
        }
    }

    pub fn item(mut self, item: MenuPopoverItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn separator(mut self) -> Self {
        self.items.push(MenuPopoverItem {
            label: "---".to_string(),
            icon: None,
            shortcut: None,
            disabled: false,
            danger: false,
        });
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl Default for MenuPopover {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for MenuPopover {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let border = hsla(0.0, 0.0, 0.2, 1.0);
        let hover = hsla(0.0, 0.0, 0.18, 1.0);
        let danger = hsla(0.0, 0.7, 0.5, 1.0);

        div()
            .w(px(self.width))
            .rounded(px(8.0))
            .bg(surface)
            .border_1()
            .border_color(border)
            .shadow_lg()
            .py_1()
            .flex()
            .flex_col()
            .children(self.items.into_iter().map(|item| {
                if item.label == "---" {
                    div()
                        .h(px(1.0))
                        .mx_2()
                        .my_1()
                        .bg(border)
                        .into_any_element()
                } else {
                    let item_text = if item.danger { danger } else { text };
                    let opacity = if item.disabled { 0.5 } else { 1.0 };

                    div()
                        .h(px(32.0))
                        .px_3()
                        .flex()
                        .items_center()
                        .gap_2()
                        .opacity(opacity)
                        .when(!item.disabled, |d| {
                            d.cursor_pointer()
                                .hover(|s| s.bg(hover))
                        })
                        .when_some(item.icon, |d, icon| {
                            d.child(
                                div()
                                    .w(px(16.0))
                                    .text_sm()
                                    .text_color(text_muted)
                                    .child(icon)
                            )
                        })
                        .child(
                            div()
                                .flex_1()
                                .text_sm()
                                .text_color(item_text)
                                .child(item.label)
                        )
                        .when_some(item.shortcut, |d, shortcut| {
                            d.child(
                                div()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .child(shortcut)
                            )
                        })
                        .into_any_element()
                }
            }))
    }
}
