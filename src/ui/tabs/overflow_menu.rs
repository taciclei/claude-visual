use gpui::prelude::*;
use gpui::*;

use super::tab_bar::TabBar;

impl TabBar {
    /// Toggle the overflow menu
    pub fn toggle_overflow_menu(&mut self, cx: &mut Context<Self>) {
        self.show_overflow_menu = !self.show_overflow_menu;
        cx.notify();
    }

    /// Hide the overflow menu
    pub fn hide_overflow_menu(&mut self, cx: &mut Context<Self>) {
        if self.show_overflow_menu {
            self.show_overflow_menu = false;
            cx.notify();
        }
    }

    /// Select a tab from the overflow menu and close the menu
    pub(super) fn select_from_overflow(&mut self, index: usize, cx: &mut Context<Self>) {
        self.select_tab(index, cx);
        self.show_overflow_menu = false;
        cx.notify();
    }

    /// Close a tab from the overflow menu
    pub(super) fn close_from_overflow(&mut self, index: usize, cx: &mut Context<Self>) {
        self.close_tab(index, cx);
        // Keep menu open if there are still tabs
        cx.notify();
    }

    /// Render the overflow menu dropdown
    pub(super) fn render_overflow_menu(
        &self,
        theme: &crate::app::theme::Theme,
        active_index: usize,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let tab_items: Vec<_> = self
            .tabs
            .iter()
            .enumerate()
            .map(|(idx, tab)| {
                (
                    idx,
                    tab.title.clone(),
                    tab.is_dirty,
                    tab.is_pinned,
                    idx == active_index,
                )
            })
            .collect();

        div()
            .id("overflow-menu")
            .absolute()
            .right_0()
            .top(px(32.0))
            .w(px(250.0))
            .max_h(px(400.0))
            .rounded_md()
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.border)
            .shadow_lg()
            .id("scroll-overflow-menu")
            .overflow_y_scroll()

            // Prevent clicks from propagating
            .on_mouse_down(MouseButton::Left, |_, _window, cx| {
            })
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text_muted)
                            .child(format!("All Tabs ({})", self.tabs.len())),
                    )
                    .child(
                        div()
                            .id("overflow-close-all")
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .text_xs()
                            .text_color(theme.colors.error)
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.colors.error.opacity(0.1)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.close_all_unpinned(cx);
                            }))
                            .child("Close All"),
                    ),
            )
            // Tab list
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_1()
                    .children(tab_items.into_iter().map(|(idx, title, is_dirty, is_pinned, is_active)| {
                        div()
                            .id(ElementId::Name(format!("overflow-tab-{}", idx).into()))
                            .flex()
                            .items_center()
                            .justify_between()
                            .w_full()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .cursor_pointer()
                            .when(is_active, |d| {
                                d.bg(theme.colors.accent.opacity(0.15))
                            })
                            .hover(|style| {
                                if is_active {
                                    style.bg(theme.colors.accent.opacity(0.2))
                                } else {
                                    style.bg(theme.colors.surface_hover)
                                }
                            })
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.select_from_overflow(idx, cx);
                            }))
                            // Left side: indicators + title
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .flex_1()
                                    .overflow_hidden()
                                    // Pin indicator
                                    .when(is_pinned, |d| {
                                        d.child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.accent)
                                                .child("📌"),
                                        )
                                    })
                                    // Dirty indicator
                                    .when(is_dirty && !is_pinned, |d| {
                                        d.child(
                                            div()
                                                .size(px(6.0))
                                                .rounded_full()
                                                .bg(theme.colors.warning),
                                        )
                                    })
                                    // Active indicator
                                    .when(is_active && !is_pinned, |d| {
                                        d.child(
                                            div()
                                                .size(px(6.0))
                                                .rounded_full()
                                                .bg(theme.colors.accent),
                                        )
                                    })
                                    // Title
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(if is_active {
                                                theme.colors.accent
                                            } else {
                                                theme.colors.text
                                            })
                                            .truncate()
                                            .child(title),
                                    ),
                            )
                            // Right side: close button (not for pinned tabs)
                            .when(!is_pinned, |d| {
                                d.child(
                                    div()
                                        .id(ElementId::Name(format!("overflow-close-{}", idx).into()))
                                        .size(px(18.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .rounded_sm()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .hover(|s| {
                                            s.bg(theme.colors.error.opacity(0.2))
                                                .text_color(theme.colors.error)
                                        })
                                        .on_click(cx.listener(move |this, event: &ClickEvent, _window, cx| {
                                            this.close_from_overflow(idx, cx);
                                        }))
                                        .child("×"),
                                )
                            })
                    })),
            )
    }
}
