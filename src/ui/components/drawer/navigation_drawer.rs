//! Navigation drawer with menu items

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Navigation drawer with menu items
#[derive(Clone)]
pub struct NavigationDrawer {
    title: String,
    items: Vec<NavDrawerItem>,
    selected: Option<usize>,
    position: DrawerPosition,
}

#[derive(Clone)]
pub struct NavDrawerItem {
    pub label: String,
    pub icon: Option<String>,
    pub badge: Option<String>,
}

impl NavigationDrawer {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
            selected: None,
            position: DrawerPosition::Left,
        }
    }

    pub fn item(mut self, label: impl Into<String>) -> Self {
        self.items.push(NavDrawerItem {
            label: label.into(),
            icon: None,
            badge: None,
        });
        self
    }

    pub fn item_with_icon(mut self, label: impl Into<String>, icon: impl Into<String>) -> Self {
        self.items.push(NavDrawerItem {
            label: label.into(),
            icon: Some(icon.into()),
            badge: None,
        });
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    pub fn position(mut self, position: DrawerPosition) -> Self {
        self.position = position;
        self
    }
}

impl RenderOnce for NavigationDrawer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        let selected_idx = self.selected;

        div()
            .w(px(280.0))
            .h_full()
            .bg(surface)
            .border_r_1()
            .border_color(border)
            .flex()
            .flex_col()
            // Header
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_4()
                    .border_b_1()
                    .border_color(border)
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .text_color(text)
                            .child(self.title),
                    ),
            )
            // Menu items
            .child(
                div()
                    .flex_1()
                    .py_2()
                    .id("scroll-nav-drawer")
                    .overflow_y_scroll()
                    .children(self.items.into_iter().enumerate().map(|(idx, item)| {
                        let is_selected = selected_idx == Some(idx);

                        div()
                            .w_full()
                            .px_3()
                            .py_2()
                            .mx_2()
                            .rounded(px(6.0))
                            .flex()
                            .items_center()
                            .gap_3()
                            .cursor_pointer()
                            .when(is_selected, |d| {
                                d.bg(accent.opacity(0.15)).text_color(accent)
                            })
                            .when(!is_selected, |d| {
                                d.text_color(text_muted)
                                    .hover(|s| s.bg(surface_hover).text_color(text))
                            })
                            .when_some(item.icon, |d, icon| {
                                d.child(div().w(px(20.0)).text_center().child(icon))
                            })
                            .child(div().flex_1().text_sm().child(item.label))
                            .when_some(item.badge, |d, badge| {
                                d.child(
                                    div()
                                        .px_2()
                                        .py(px(2.0))
                                        .rounded(px(10.0))
                                        .bg(accent)
                                        .text_xs()
                                        .text_color(gpui::white())
                                        .child(badge),
                                )
                            })
                    })),
            )
    }
}
