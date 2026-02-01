//! Pagination render implementation

use gpui::prelude::*;
use gpui::*;

use super::component::Pagination;
use super::types::*;

impl Render for Pagination {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let button_size = self.size.button_size();
        let font_size = self.size.font_size();

        let is_simple = matches!(self.style, PaginationStyle::Simple);
        let is_pill = matches!(self.style, PaginationStyle::Pill);
        let is_outlined = matches!(self.style, PaginationStyle::Outlined);

        let opacity = if self.disabled { 0.5 } else { 1.0 };

        let pages = self.visible_range();

        // Copy theme colors for move closures
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let border_color = theme.colors.border;
        let surface_hover = theme.colors.surface_hover;
        let accent_color = theme.colors.accent;

        div()
            .id("pagination")
            .flex()
            .items_center()
            .gap_1()
            .opacity(opacity)
            // First button
            .when(self.show_first_last, |d| {
                let can_go = self.current_page > 1 && !self.disabled;
                let first_listener = cx.listener(|this, _, _window, cx| {
                    this.first(cx);
                });
                d.child(
                    div()
                        .id("pagination-first")
                        .size(px(button_size))
                        .rounded(if is_pill {
                            px(button_size / 2.0)
                        } else {
                            px(6.0)
                        })
                        .when(is_outlined, |d| d.border_1().border_color(border_color))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(font_size))
                        .text_color(if can_go { text_color } else { text_muted })
                        .when(can_go, |d| {
                            d.cursor_pointer()
                                .hover(move |s| s.bg(surface_hover))
                                .on_click(first_listener)
                        })
                        .child("«"),
                )
            })
            // Previous button
            .when(self.show_prev_next, |d| {
                let can_go = self.current_page > 1 && !self.disabled;
                let prev_listener = cx.listener(|this, _, _window, cx| {
                    this.prev(cx);
                });
                d.child(
                    div()
                        .id("pagination-prev")
                        .size(px(button_size))
                        .rounded(if is_pill {
                            px(button_size / 2.0)
                        } else {
                            px(6.0)
                        })
                        .when(is_outlined, |d| d.border_1().border_color(border_color))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(font_size))
                        .text_color(if can_go { text_color } else { text_muted })
                        .when(can_go, |d| {
                            d.cursor_pointer()
                                .hover(move |s| s.bg(surface_hover))
                                .on_click(prev_listener)
                        })
                        .child("‹"),
                )
            })
            // Page numbers
            .when(!is_simple, |d| {
                d.children(pages.iter().enumerate().map(|(idx, page)| {
                    match page {
                        Some(num) => {
                            let is_current = *num == self.current_page;
                            let page_num = *num;

                            div()
                                .id(SharedString::from(format!("pagination-page-{}", idx)))
                                .size(px(button_size))
                                .rounded(if is_pill {
                                    px(button_size / 2.0)
                                } else {
                                    px(6.0)
                                })
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(font_size))
                                .when(is_current, |d| d.bg(accent_color).text_color(gpui::white()))
                                .when(!is_current, |d| {
                                    d.text_color(text_color)
                                        .when(is_outlined, |d| {
                                            d.border_1().border_color(border_color)
                                        })
                                        .when(!self.disabled, |d| {
                                            let page_listener =
                                                cx.listener(move |this, _, _window, cx| {
                                                    this.set_page(page_num, cx);
                                                });
                                            d.cursor_pointer()
                                                .hover(move |s| s.bg(surface_hover))
                                                .on_click(page_listener)
                                        })
                                })
                                .child(num.to_string())
                                .into_any_element()
                        }
                        None => div()
                            .id(SharedString::from(format!("pagination-ellipsis-{}", idx)))
                            .size(px(button_size))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(font_size))
                            .text_color(text_muted)
                            .child("…")
                            .into_any_element(),
                    }
                }))
            })
            // Simple style: "Page X of Y"
            .when(is_simple, |d| {
                d.child(
                    div()
                        .text_size(px(font_size))
                        .text_color(text_color)
                        .child(format!(
                            "Page {} of {}",
                            self.current_page, self.total_pages
                        )),
                )
            })
            // Next button
            .when(self.show_prev_next, |d| {
                let can_go = self.current_page < self.total_pages && !self.disabled;
                let next_listener = cx.listener(|this, _, _window, cx| {
                    this.next(cx);
                });
                d.child(
                    div()
                        .id("pagination-next")
                        .size(px(button_size))
                        .rounded(if is_pill {
                            px(button_size / 2.0)
                        } else {
                            px(6.0)
                        })
                        .when(is_outlined, |d| d.border_1().border_color(border_color))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(font_size))
                        .text_color(if can_go { text_color } else { text_muted })
                        .when(can_go, |d| {
                            d.cursor_pointer()
                                .hover(move |s| s.bg(surface_hover))
                                .on_click(next_listener)
                        })
                        .child("›"),
                )
            })
            // Last button
            .when(self.show_first_last, |d| {
                let can_go = self.current_page < self.total_pages && !self.disabled;
                let last_listener = cx.listener(|this, _, _window, cx| {
                    this.last(cx);
                });
                d.child(
                    div()
                        .id("pagination-last")
                        .size(px(button_size))
                        .rounded(if is_pill {
                            px(button_size / 2.0)
                        } else {
                            px(6.0)
                        })
                        .when(is_outlined, |d| d.border_1().border_color(border_color))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(font_size))
                        .text_color(if can_go { text_color } else { text_muted })
                        .when(can_go, |d| {
                            d.cursor_pointer()
                                .hover(move |s| s.bg(surface_hover))
                                .on_click(last_listener)
                        })
                        .child("»"),
                )
            })
    }
}
