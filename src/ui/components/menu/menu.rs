//! Main Menu component

use std::sync::Arc;

use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;
use super::types::*;

/// Menu component
pub struct Menu {
    app_state: Arc<AppState>,
    /// Menu items
    items: Vec<MenuItemData>,
    /// Whether menu is visible
    visible: bool,
    /// Hovered item index
    hovered_index: Option<usize>,
    /// Min width
    min_width: f32,
}

impl Menu {
    pub fn new(app_state: Arc<AppState>, items: Vec<MenuItemData>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            items,
            visible: true,
            hovered_index: None,
            min_width: 200.0,
        }
    }

    /// Set items
    pub fn set_items(&mut self, items: Vec<MenuItemData>, cx: &mut Context<Self>) {
        self.items = items;
        cx.notify();
    }

    /// Show menu
    pub fn show(&mut self, cx: &mut Context<Self>) {
        self.visible = true;
        self.hovered_index = None;
        cx.notify();
    }

    /// Hide menu
    pub fn hide(&mut self, cx: &mut Context<Self>) {
        self.visible = false;
        cx.emit(MenuEvent::Close);
        cx.notify();
    }

    /// Set min width
    pub fn set_min_width(&mut self, width: f32, cx: &mut Context<Self>) {
        self.min_width = width;
        cx.notify();
    }

    /// Select item
    fn select_item(&mut self, id: String, cx: &mut Context<Self>) {
        cx.emit(MenuEvent::Select(id));
        self.hide(cx);
    }
}

impl EventEmitter<MenuEvent> for Menu {}

impl Render for Menu {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        if !self.visible {
            return div().id("menu-hidden");
        }

        div()
            .id("menu")
            .min_w(px(self.min_width))
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
                    MenuItemType::Separator => {
                        div()
                            .id(SharedString::from(format!("menu-sep-{}", idx)))
                            .h(px(1.0))
                            .mx_2()
                            .my_1()
                            .bg(theme.colors.border)
                    }
                    MenuItemType::Header => {
                        div()
                            .id(SharedString::from(format!("menu-header-{}", idx)))
                            .px_3()
                            .py_1()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .font_weight(FontWeight::MEDIUM)
                            .child(item.label.clone())
                    }
                    MenuItemType::Item => {
                        let item_id = item.id.clone();
                        let is_disabled = item.disabled;
                        let is_destructive = item.destructive;

                        let text_color = if is_destructive {
                            theme.colors.error
                        } else {
                            theme.colors.text
                        };

                        div()
                            .id(SharedString::from(format!("menu-item-{}", idx)))
                            .px_3()
                            .py_2()
                            .mx_1()
                            .rounded(px(4.0))
                            .flex()
                            .items_center()
                            .gap_3()
                            .text_sm()
                            .text_color(if is_disabled { theme.colors.text_muted } else { text_color })
                            .when(!is_disabled, |d| {
                                d.cursor_pointer()
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                            })
                            .when(!is_disabled, |d| {
                                d.on_click(cx.listener(move |this, _, _window, cx| {
                                    this.select_item(item_id.clone(), cx);
                                }))
                            })
                            // Checkbox/radio indicator
                            .when_some(item.checked, |d, checked| {
                                d.child(
                                    div()
                                        .w(px(16.0))
                                        .text_color(theme.colors.accent)
                                        .child(if checked { "✓" } else { "" })
                                )
                            })
                            // Icon
                            .when_some(item.icon.clone(), |d, icon| {
                                d.child(
                                    div()
                                        .w(px(16.0))
                                        .text_color(theme.colors.text_muted)
                                        .child(icon)
                                )
                            })
                            // Label
                            .child(
                                div()
                                    .flex_1()
                                    .child(item.label.clone())
                            )
                            // Shortcut
                            .when_some(item.shortcut.clone(), |d, shortcut| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child(shortcut)
                                )
                            })
                    }
                    MenuItemType::Submenu(_) => {
                        // Submenu indicator
                        div()
                            .id(SharedString::from(format!("menu-submenu-{}", idx)))
                            .px_3()
                            .py_2()
                            .mx_1()
                            .rounded(px(4.0))
                            .flex()
                            .items_center()
                            .gap_3()
                            .text_sm()
                            .text_color(theme.colors.text)
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.colors.surface_hover))
                            // Icon
                            .when_some(item.icon.clone(), |d, icon| {
                                d.child(
                                    div()
                                        .w(px(16.0))
                                        .text_color(theme.colors.text_muted)
                                        .child(icon)
                                )
                            })
                            // Label
                            .child(
                                div()
                                    .flex_1()
                                    .child(item.label.clone())
                            )
                            // Submenu arrow
                            .child(
                                div()
                                    .text_color(theme.colors.text_muted)
                                    .child("▸")
                            )
                    }
                }
            }))
    }
}
