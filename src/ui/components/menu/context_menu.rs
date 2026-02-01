//! Context menu component

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// Context menu wrapper
pub struct ContextMenu {
    app_state: Arc<AppState>,
    /// Menu items
    items: Vec<MenuItemData>,
    /// Whether menu is visible
    visible: bool,
    /// Position X
    x: f32,
    /// Position Y
    y: f32,
}

impl ContextMenu {
    pub fn new(
        app_state: Arc<AppState>,
        items: Vec<MenuItemData>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            items,
            visible: false,
            x: 0.0,
            y: 0.0,
        }
    }

    /// Show at position
    pub fn show_at(&mut self, x: f32, y: f32, cx: &mut Context<Self>) {
        self.x = x;
        self.y = y;
        self.visible = true;
        cx.notify();
    }

    /// Hide menu
    pub fn hide(&mut self, cx: &mut Context<Self>) {
        self.visible = false;
        cx.notify();
    }

    /// Set items
    pub fn set_items(&mut self, items: Vec<MenuItemData>, cx: &mut Context<Self>) {
        self.items = items;
        cx.notify();
    }

    fn select_item(&mut self, id: String, cx: &mut Context<Self>) {
        cx.emit(ContextMenuEvent::Select(id));
        self.hide(cx);
    }
}

impl EventEmitter<ContextMenuEvent> for ContextMenu {}

impl Render for ContextMenu {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        if !self.visible {
            return div().id("context-menu-hidden");
        }

        div()
            .id("context-menu")
            .absolute()
            .left(px(self.x))
            .top(px(self.y))
            .min_w(px(180.0))
            .py_1()
            .rounded(px(8.0))
            .border_1()
            .border_color(theme.colors.border)
            .bg(theme.colors.surface)
            .shadow_lg()
            .flex()
            .flex_col()
            .children(self.items.iter().enumerate().map(|(idx, item)| {
                match &item.item_type {
                    MenuItemType::Separator => div()
                        .id(SharedString::from(format!("ctx-sep-{}", idx)))
                        .h(px(1.0))
                        .mx_2()
                        .my_1()
                        .bg(theme.colors.border),
                    MenuItemType::Header => div()
                        .id(SharedString::from(format!("ctx-header-{}", idx)))
                        .px_3()
                        .py_1()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .font_weight(FontWeight::MEDIUM)
                        .child(item.label.clone()),
                    _ => {
                        let item_id = item.id.clone();
                        let is_disabled = item.disabled;
                        let is_destructive = item.destructive;

                        let text_color = if is_destructive {
                            theme.colors.error
                        } else {
                            theme.colors.text
                        };

                        div()
                            .id(SharedString::from(format!("ctx-item-{}", idx)))
                            .px_3()
                            .py_2()
                            .mx_1()
                            .rounded(px(4.0))
                            .flex()
                            .items_center()
                            .gap_3()
                            .text_sm()
                            .text_color(if is_disabled {
                                theme.colors.text_muted
                            } else {
                                text_color
                            })
                            .when(!is_disabled, |d| {
                                d.cursor_pointer()
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                            })
                            .when(!is_disabled, |d| {
                                d.on_click(cx.listener(move |this, _, _window, cx| {
                                    this.select_item(item_id.clone(), cx);
                                }))
                            })
                            .when_some(item.icon.clone(), |d, icon| {
                                d.child(div().w(px(16.0)).child(icon))
                            })
                            .child(div().flex_1().child(item.label.clone()))
                            .when_some(item.shortcut.clone(), |d, shortcut| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child(shortcut),
                                )
                            })
                    }
                }
            }))
    }
}
