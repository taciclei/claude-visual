//! Action menu component (dropdown style)

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// Simple action menu (dropdown style)
pub struct ActionMenu {
    app_state: Arc<AppState>,
    /// Button label
    label: String,
    /// Button icon
    icon: Option<String>,
    /// Menu items
    items: Vec<MenuItemData>,
    /// Whether dropdown is open
    open: bool,
}

impl ActionMenu {
    pub fn new(
        app_state: Arc<AppState>,
        label: impl Into<String>,
        items: Vec<MenuItemData>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            label: label.into(),
            icon: None,
            items,
            open: false,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn set_items(&mut self, items: Vec<MenuItemData>, cx: &mut Context<Self>) {
        self.items = items;
        cx.notify();
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        self.open = !self.open;
        cx.notify();
    }

    pub fn close(&mut self, cx: &mut Context<Self>) {
        self.open = false;
        cx.notify();
    }

    fn select_item(&mut self, id: String, cx: &mut Context<Self>) {
        cx.emit(ActionMenuEvent::Select(id));
        self.close(cx);
    }
}

impl EventEmitter<ActionMenuEvent> for ActionMenu {}

impl Render for ActionMenu {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .id("action-menu")
            .relative()
            // Trigger button
            .child(
                div()
                    .id("action-menu-trigger")
                    .px_3()
                    .py_2()
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(theme.colors.border)
                    .bg(theme.colors.surface)
                    .flex()
                    .items_center()
                    .gap_2()
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.colors.surface_hover))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle(cx);
                    }))
                    .when_some(self.icon.clone(), |d, icon| d.child(div().child(icon)))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text)
                            .child(self.label.clone()),
                    )
                    .child(
                        div()
                            .text_color(theme.colors.text_muted)
                            .text_xs()
                            .child(if self.open { "▲" } else { "▼" }),
                    ),
            )
            // Dropdown menu
            .when(self.open, |d| {
                d.child(
                    div()
                        .absolute()
                        .top(px(40.0))
                        .left_0()
                        .min_w(px(180.0))
                        .py_1()
                        .rounded(px(8.0))
                        .border_1()
                        .border_color(theme.colors.border)
                        .bg(theme.colors.surface)
                        .shadow_lg()
                        .flex()
                        .flex_col()
                        .children(self.items.iter().enumerate().filter_map(|(idx, item)| {
                            match &item.item_type {
                                MenuItemType::Separator => Some(
                                    div()
                                        .id(SharedString::from(format!("separator-{}", idx)))
                                        .h(px(1.0))
                                        .mx_2()
                                        .my_1()
                                        .bg(theme.colors.border),
                                ),
                                MenuItemType::Item => {
                                    let item_id = item.id.clone();
                                    let is_disabled = item.disabled;

                                    Some(
                                        div()
                                            .id(SharedString::from(format!("action-item-{}", idx)))
                                            .px_3()
                                            .py_2()
                                            .mx_1()
                                            .rounded(px(4.0))
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .text_sm()
                                            .text_color(if is_disabled {
                                                theme.colors.text_muted
                                            } else {
                                                theme.colors.text
                                            })
                                            .when(!is_disabled, |d| {
                                                d.cursor_pointer()
                                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                            })
                                            .when(!is_disabled, |d| {
                                                d.on_click(cx.listener(
                                                    move |this, _, _window, cx| {
                                                        this.select_item(item_id.clone(), cx);
                                                    },
                                                ))
                                            })
                                            .when_some(item.icon.clone(), |d, icon| {
                                                d.child(div().child(icon))
                                            })
                                            .child(item.label.clone()),
                                    )
                                }
                                _ => None,
                            }
                        })),
                )
            })
    }
}
