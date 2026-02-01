use gpui::prelude::*;
use gpui::*;

use super::tab_bar::TabBar;
use super::types::*;

impl Render for TabBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let active_index = self.active_index;
        let tab_count = self.tabs.len();

        // Pre-compute tab data
        let tab_data: Vec<_> = self
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
            .id("tab-bar")
            .track_focus(&self.focus_handle)
            .flex_shrink_0()
            .h(px(36.0))
            .w_full()
            .bg(theme.colors.surface)
            .border_b_1()
            .border_color(theme.colors.border)
            .flex()
            .flex_row()
            .items_center()
            .id("scroll-tab-bar")
            .overflow_x_scroll()
            // Tabs container
            .child(
                div().flex_1().flex().flex_row().gap_px().px_1().children(
                    tab_data
                        .into_iter()
                        .map(|(idx, title, is_dirty, is_pinned, is_active)| {
                            let bg = if is_active {
                                theme.colors.background
                            } else {
                                theme.colors.surface
                            };
                            let hover_bg = if is_active {
                                theme.colors.background
                            } else {
                                theme.colors.surface_hover
                            };
                            let text_color = if is_active {
                                theme.colors.text
                            } else {
                                theme.colors.text_muted
                            };

                            div()
                                .id(ElementId::Name(format!("tab-{}", idx).into()))
                                .group("tab")
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .h(px(28.0))
                                .px_3()
                                .rounded_t_md()
                                .bg(bg)
                                .hover(|style| style.bg(hover_bg))
                                .cursor_pointer()
                                .when(is_pinned, |this| {
                                    this.border_l_2().border_color(theme.colors.accent)
                                })
                                .when(is_active, |this| {
                                    this.border_t_1()
                                        .border_l_1()
                                        .border_r_1()
                                        .border_color(theme.colors.border)
                                })
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    this.select_tab(idx, cx);
                                }))
                                // Drag support (only for unpinned tabs)
                                .when(!is_pinned, |this| {
                                    this.on_drag(
                                        DraggedTab {
                                            index: idx,
                                            tab: Tab {
                                                id: format!("tab-{}", idx),
                                                title: title.clone(),
                                                conversation_id: None,
                                                is_dirty,
                                                is_pinned,
                                            },
                                        },
                                        |dragged, _offset, _window, cx| {
                                            let title = dragged.tab.title.clone();
                                            cx.new(|_| TabDragPreview { title })
                                        },
                                    )
                                    .drag_over::<DraggedTab>(|style, _, _window, _cx| {
                                        style.bg(hsla(210.0 / 360.0, 0.5, 0.5, 0.3))
                                    })
                                    .on_drop(cx.listener(
                                        move |this, _: &DraggedTab, _window, cx| {
                                            this.handle_drag_over(idx, cx);
                                            this.end_drag(cx);
                                        },
                                    ))
                                })
                                // Tab content
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        // Pin indicator
                                        .when(is_pinned, |this| {
                                            this.child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.colors.accent)
                                                    .child("📌"),
                                            )
                                        })
                                        // Dirty indicator (only show if not pinned)
                                        .when(is_dirty && !is_pinned, |this| {
                                            this.child(
                                                div()
                                                    .size(px(6.0))
                                                    .rounded_full()
                                                    .bg(theme.colors.warning),
                                            )
                                        })
                                        // Title (shorter for pinned tabs)
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(text_color)
                                                .max_w(if is_pinned { px(80.0) } else { px(120.0) })
                                                .truncate()
                                                .child(title),
                                        ),
                                )
                                // Close/unpin button
                                .child(
                                    div()
                                        .id(ElementId::Name(format!("close-tab-{}", idx).into()))
                                        .size(px(16.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .rounded_sm()
                                        .when(is_pinned, |this| {
                                            // Pinned tabs show subtle unpin button
                                            this.hover(|style| {
                                                style.bg(theme.colors.accent.opacity(0.2))
                                            })
                                        })
                                        .when(!is_pinned, |this| {
                                            // Unpinned tabs show close button
                                            this.hover(|style| {
                                                style.bg(theme.colors.error.opacity(0.2))
                                            })
                                        })
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .when(!is_pinned && (tab_count > 1 || is_dirty), |this| {
                                            this.visible()
                                        })
                                        .on_click(cx.listener(
                                            move |this, event: &ClickEvent, _window, cx| {
                                                if is_pinned {
                                                    // Clicking close on pinned tab unpins it
                                                    this.unpin_tab(idx, cx);
                                                } else {
                                                    this.close_tab(idx, cx);
                                                }
                                            },
                                        ))
                                        .child(if is_pinned { "⊗" } else { "×" }),
                                )
                        }),
                ),
            )
            // New tab button
            .child(
                div()
                    .id("new-tab-button")
                    .flex_shrink_0()
                    .size(px(28.0))
                    .mx_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_md()
                    .hover(|style| style.bg(theme.colors.surface_hover))
                    .cursor_pointer()
                    .text_sm()
                    .text_color(theme.colors.text_muted)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.add_tab(cx);
                    }))
                    .child("+"),
            )
            // Overflow menu button (shown when there are multiple tabs)
            .when(tab_count > 1, |d| {
                d.child(
                    div()
                        .id("overflow-menu-container")
                        .relative()
                        .flex_shrink_0()
                        .child(
                            div()
                                .id("overflow-menu-button")
                                .size(px(28.0))
                                .mr_1()
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded_md()
                                .when(self.show_overflow_menu, |d| {
                                    d.bg(theme.colors.accent.opacity(0.2))
                                })
                                .hover(|style| style.bg(theme.colors.surface_hover))
                                .cursor_pointer()
                                .text_sm()
                                .text_color(if self.show_overflow_menu {
                                    theme.colors.accent
                                } else {
                                    theme.colors.text_muted
                                })
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.toggle_overflow_menu(cx);
                                }))
                                .child("▼"),
                        )
                        // Overflow dropdown menu
                        .when(self.show_overflow_menu, |d| {
                            d.child(self.render_overflow_menu(theme, active_index, cx))
                        }),
                )
            })
    }
}
