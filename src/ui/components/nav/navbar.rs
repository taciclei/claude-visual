use gpui::*;
use gpui::prelude::*;

use super::types::NavItem;

/// Navbar component - top navigation bar
#[derive(IntoElement)]
pub struct Navbar {
    id: ElementId,
    logo: Option<gpui::AnyElement>,
    items: Vec<NavItem>,
    actions: Vec<gpui::AnyElement>,
    sticky: bool,
    transparent: bool,
    height: f32,
    background: Option<gpui::Hsla>,
    border_color: Option<gpui::Hsla>,
}

impl Navbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            logo: None,
            items: Vec::new(),
            actions: Vec::new(),
            sticky: false,
            transparent: false,
            height: 56.0,
            background: None,
            border_color: None,
        }
    }

    pub fn logo(mut self, logo: impl IntoElement) -> Self {
        self.logo = Some(logo.into_any_element());
        self
    }

    pub fn items(mut self, items: Vec<NavItem>) -> Self {
        self.items = items;
        self
    }

    pub fn actions(mut self, actions: Vec<impl IntoElement>) -> Self {
        self.actions = actions.into_iter().map(|a| a.into_any_element()).collect();
        self
    }

    pub fn sticky(mut self, sticky: bool) -> Self {
        self.sticky = sticky;
        self
    }

    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border_color(mut self, color: gpui::Hsla) -> Self {
        self.border_color = Some(color);
        self
    }
}

impl RenderOnce for Navbar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = if self.transparent {
            hsla(0.0, 0.0, 0.0, 0.0)
        } else {
            self.background.unwrap_or(hsla(0.0, 0.0, 0.1, 1.0))
        };
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.2, 1.0));

        div()
            .id(self.id)
            .w_full()
            .h(px(self.height))
            .px(px(16.0))
            .flex()
            .items_center()
            .justify_between()
            .bg(bg)
            .border_b_1()
            .border_color(border)
            .child(
                // Left section: logo + nav items
                div()
                    .flex()
                    .items_center()
                    .gap(px(24.0))
                    .when(self.logo.is_some(), |el| {
                        el.child(self.logo.unwrap())
                    })
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .children(self.items.into_iter().map(|item| {
                                div()
                                    .px(px(12.0))
                                    .py(px(8.0))
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .rounded(px(6.0))
                                    .cursor_pointer()
                                    .when(item.active, |el| {
                                        el.bg(hsla(0.0, 0.0, 0.2, 1.0))
                                    })
                                    .when(!item.active && !item.disabled, |el| {
                                        el.hover(|style| style.bg(hsla(0.0, 0.0, 0.15, 1.0)))
                                    })
                                    .when(item.disabled, |el| {
                                        el.opacity(0.5).cursor_not_allowed()
                                    })
                                    .when(item.icon.is_some(), |el| {
                                        el.child(
                                            div()
                                                .text_size(px(14.0))
                                                .child(item.icon.clone().unwrap_or_default())
                                        )
                                    })
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(if item.active {
                                                hsla(0.0, 0.0, 1.0, 1.0)
                                            } else {
                                                hsla(0.0, 0.0, 0.7, 1.0)
                                            })
                                            .child(item.label.clone())
                                    )
                            }))
                    )
            )
            .child(
                // Right section: actions
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .children(self.actions)
            )
    }
}
