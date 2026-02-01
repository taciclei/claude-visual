use gpui::*;
use gpui::prelude::*;

use super::types::NavSection;

/// Sidebar navigation component
#[derive(IntoElement)]
pub struct SidebarNav {
    id: ElementId,
    header: Option<gpui::AnyElement>,
    sections: Vec<NavSection>,
    footer: Option<gpui::AnyElement>,
    collapsed: bool,
    width: f32,
    collapsed_width: f32,
    background: Option<gpui::Hsla>,
    border_color: Option<gpui::Hsla>,
}

impl SidebarNav {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            header: None,
            sections: Vec::new(),
            footer: None,
            collapsed: false,
            width: 240.0,
            collapsed_width: 64.0,
            background: None,
            border_color: None,
        }
    }

    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub fn sections(mut self, sections: Vec<NavSection>) -> Self {
        self.sections = sections;
        self
    }

    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn collapsed_width(mut self, width: f32) -> Self {
        self.collapsed_width = width;
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

impl RenderOnce for SidebarNav {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.08, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.2, 1.0));
        let width = if self.collapsed { self.collapsed_width } else { self.width };

        div()
            .id(self.id)
            .w(px(width))
            .h_full()
            .flex()
            .flex_col()
            .bg(bg)
            .border_r_1()
            .border_color(border)
            .child(
                // Header
                div()
                    .when(self.header.is_some(), |el| {
                        el.p(px(12.0))
                            .border_b_1()
                            .border_color(border)
                            .child(self.header.unwrap())
                    })
            )
            .child(
                // Sections
                div()
                    .flex_1()
                    .id("scroll-sidebar-sections")
                    .overflow_y_scroll()
                    .py(px(8.0))
                    .children(self.sections.into_iter().map(|section| {
                        div()
                            .flex()
                            .flex_col()
                            .mb(px(8.0))
                            .when(section.title.is_some() && !self.collapsed, |el| {
                                el.child(
                                    div()
                                        .px(px(16.0))
                                        .py(px(8.0))
                                        .text_size(px(11.0))
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                        .child(section.title.unwrap_or_default())
                                )
                            })
                            .children(section.items.into_iter().map(|item| {
                                let is_active = item.active;

                                div()
                                    .mx(px(8.0))
                                    .px(px(if self.collapsed { 8.0 } else { 12.0 }))
                                    .h(px(36.0))
                                    .flex()
                                    .items_center()
                                    .when(self.collapsed, |el| el.justify_center())
                                    .gap(px(10.0))
                                    .rounded(px(6.0))
                                    .cursor_pointer()
                                    .when(is_active, |el| {
                                        el.bg(hsla(0.0, 0.0, 0.15, 1.0))
                                    })
                                    .when(!is_active && !item.disabled, |el| {
                                        el.hover(|style| style.bg(hsla(0.0, 0.0, 0.12, 1.0)))
                                    })
                                    .when(item.disabled, |el| {
                                        el.opacity(0.5).cursor_not_allowed()
                                    })
                                    .when(item.icon.is_some(), |el| {
                                        el.child(
                                            div()
                                                .text_size(px(16.0))
                                                .text_color(if is_active {
                                                    hsla(0.6, 0.7, 0.6, 1.0)
                                                } else {
                                                    hsla(0.0, 0.0, 0.5, 1.0)
                                                })
                                                .child(item.icon.clone().unwrap_or_default())
                                        )
                                    })
                                    .when(!self.collapsed, |el| {
                                        el.child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(if is_active {
                                                    hsla(0.0, 0.0, 0.95, 1.0)
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
                            }))
                    }))
            )
            .child(
                // Footer
                div()
                    .when(self.footer.is_some(), |el| {
                        el.p(px(12.0))
                            .border_t_1()
                            .border_color(border)
                            .child(self.footer.unwrap())
                    })
            )
    }
}
