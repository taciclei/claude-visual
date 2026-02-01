use gpui::*;
use gpui::prelude::*;

use super::types::{NavItem, NavItemVariant, NavOrientation, NavSize};

/// Main navigation component
#[derive(IntoElement)]
pub struct Nav {
    id: ElementId,
    items: Vec<NavItem>,
    orientation: NavOrientation,
    size: NavSize,
    variant: NavItemVariant,
    collapsed: bool,
    background: Option<gpui::Hsla>,
    active_color: Option<gpui::Hsla>,
}

impl Nav {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            orientation: NavOrientation::default(),
            size: NavSize::default(),
            variant: NavItemVariant::default(),
            collapsed: false,
            background: None,
            active_color: None,
        }
    }

    pub fn items(mut self, items: Vec<NavItem>) -> Self {
        self.items = items;
        self
    }

    pub fn orientation(mut self, orientation: NavOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn size(mut self, size: NavSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: NavItemVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn active_color(mut self, color: gpui::Hsla) -> Self {
        self.active_color = Some(color);
        self
    }

    fn get_size_styles(&self) -> (f32, f32, f32) {
        match self.size {
            NavSize::Small => (28.0, 8.0, 12.0),
            NavSize::Medium => (36.0, 12.0, 14.0),
            NavSize::Large => (44.0, 16.0, 16.0),
        }
    }
}

impl RenderOnce for Nav {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (item_height, padding_x, font_size) = self.get_size_styles();
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.0, 0.0));
        let active_color = self.active_color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0));

        let is_vertical = self.orientation == NavOrientation::Vertical;

        div()
            .id(self.id)
            .when(is_vertical, |el| el.flex().flex_col())
            .when(!is_vertical, |el| el.flex().flex_row().items_center())
            .gap(px(4.0))
            .bg(bg)
            .children(self.items.into_iter().map(|item| {
                let is_active = item.active;

                let mut nav_item = div()
                    .h(px(item_height))
                    .px(px(padding_x))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .cursor_pointer()
                    .rounded(px(6.0));

                // Apply variant-specific styles
                nav_item = match self.variant {
                    NavItemVariant::Default => {
                        nav_item
                            .when(is_active, |el| {
                                el.bg(hsla(0.0, 0.0, 0.2, 1.0))
                            })
                            .when(!is_active && !item.disabled, |el| {
                                el.hover(|style| style.bg(hsla(0.0, 0.0, 0.15, 1.0)))
                            })
                    }
                    NavItemVariant::Subtle => {
                        nav_item
                            .when(is_active, |el| {
                                el.text_color(active_color)
                            })
                            .when(!is_active && !item.disabled, |el| {
                                el.hover(|style| style.text_color(hsla(0.0, 0.0, 0.8, 1.0)))
                            })
                    }
                    NavItemVariant::Pill => {
                        nav_item
                            .when(is_active, |el| {
                                el.bg(active_color).text_color(hsla(0.0, 0.0, 0.0, 1.0))
                            })
                            .when(!is_active && !item.disabled, |el| {
                                el.hover(|style| style.bg(hsla(0.0, 0.0, 0.15, 1.0)))
                            })
                    }
                    NavItemVariant::Underline => {
                        nav_item
                            .rounded_none()
                            .when(is_active, |el| {
                                el.border_b_2().border_color(active_color)
                            })
                            .when(!is_active && !item.disabled, |el| {
                                el.hover(|style| {
                                    style.border_b_2().border_color(hsla(0.0, 0.0, 0.3, 1.0))
                                })
                            })
                    }
                };

                nav_item
                    .when(item.disabled, |el| {
                        el.opacity(0.5).cursor_not_allowed()
                    })
                    .when(item.icon.is_some() && !self.collapsed, |el| {
                        el.child(
                            div()
                                .text_size(px(font_size))
                                .text_color(if is_active {
                                    active_color
                                } else {
                                    hsla(0.0, 0.0, 0.6, 1.0)
                                })
                                .child(item.icon.clone().unwrap_or_default())
                        )
                    })
                    .when(item.icon.is_some() && self.collapsed, |el| {
                        el.child(
                            div()
                                .text_size(px(font_size + 2.0))
                                .text_color(if is_active {
                                    active_color
                                } else {
                                    hsla(0.0, 0.0, 0.6, 1.0)
                                })
                                .child(item.icon.clone().unwrap_or_default())
                        )
                    })
                    .when(!self.collapsed, |el| {
                        el.child(
                            div()
                                .text_size(px(font_size))
                                .text_color(if is_active {
                                    hsla(0.0, 0.0, 1.0, 1.0)
                                } else {
                                    hsla(0.0, 0.0, 0.7, 1.0)
                                })
                                .child(item.label.clone())
                        )
                    })
                    .when(item.badge.is_some() && !self.collapsed, |el| {
                        el.child(
                            div()
                                .ml_auto()
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(10.0))
                                .bg(hsla(0.0, 0.7, 0.5, 1.0))
                                .text_size(px(10.0))
                                .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                .child(item.badge.unwrap_or_default())
                        )
                    })
                    .when(!item.children.is_empty() && !self.collapsed, |el| {
                        el.child(
                            div()
                                .ml_auto()
                                .text_size(px(10.0))
                                .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                .child("▶")
                        )
                    })
            }))
    }
}
