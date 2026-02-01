//! Collapsible sidebar layout component

use gpui::*;
use gpui::prelude::*;

use super::types::SidebarPosition;

/// Collapsible sidebar layout
#[derive(Clone)]
pub struct CollapsibleSidebar {
    pub(crate) width: f32,
    pub(crate) collapsed: bool,
    pub(crate) position: SidebarPosition,
}

impl CollapsibleSidebar {
    pub fn new(width: f32) -> Self {
        Self {
            width,
            collapsed: false,
            position: SidebarPosition::default(),
        }
    }

    pub fn collapsed(mut self) -> Self {
        self.collapsed = true;
        self
    }

    pub fn right(mut self) -> Self {
        self.position = SidebarPosition::Right;
        self
    }
}

impl RenderOnce for CollapsibleSidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        let is_left = matches!(self.position, SidebarPosition::Left);

        div()
            .w_full()
            .h_full()
            .flex()
            .flex_row()
            // Sidebar (left position)
            .when(is_left && !self.collapsed, |d| {
                d.child(
                    div()
                        .w(px(self.width))
                        .h_full()
                        .bg(surface)
                        .border_r_1()
                        .border_color(border)
                        .flex_shrink_0()
                )
            })
            // Collapse button (left)
            .when(is_left, |d| {
                d.child(
                    div()
                        .w(px(20.0))
                        .h_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_muted)
                                .child(if self.collapsed { "▶" } else { "◀" })
                        )
                )
            })
            // Main content
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .overflow_hidden()
            )
            // Collapse button (right)
            .when(!is_left, |d| {
                d.child(
                    div()
                        .w(px(20.0))
                        .h_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_muted)
                                .child(if self.collapsed { "◀" } else { "▶" })
                        )
                )
            })
            // Sidebar (right position)
            .when(!is_left && !self.collapsed, |d| {
                d.child(
                    div()
                        .w(px(self.width))
                        .h_full()
                        .bg(surface)
                        .border_l_1()
                        .border_color(border)
                        .flex_shrink_0()
                )
            })
    }
}
