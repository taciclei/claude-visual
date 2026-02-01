//! Search bar render functions for ChatView

use super::super::core::ChatView;
use super::super::types::MessageFilter;
use crate::claude::message::MessageRole;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    pub fn render_search_bar(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        let result_count = self.search_result_count();
        let current_index = self.current_result_index();
        let query = self.search.query.clone();
        let query_display = if query.is_empty() {
            "(type to search)".to_string()
        } else {
            query.clone()
        };
        let case_sensitive = self.search.case_sensitive;
        let regex_enabled = self.search.regex;
        let role_filter = self.search.role_filter;

        div()
            .flex()
            .flex_col()
            .bg(theme.colors.surface)
            .border_b_1()
            .border_color(theme.colors.border)
            // Main search row
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .px_4()
                    .py_2()
                    // Search input
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .flex_1()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .child("🔍"),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .bg(theme.colors.background)
                                    .border_1()
                                    .border_color(if regex_enabled {
                                        theme.colors.accent
                                    } else {
                                        theme.colors.border
                                    })
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .child(query_display),
                            ),
                    )
                    // Filter buttons row
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            // Case sensitive toggle
                            .child(
                                div()
                                    .id("search-case")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .bg(if case_sensitive {
                                        theme.colors.accent.opacity(0.2)
                                    } else {
                                        theme.colors.surface
                                    })
                                    .text_color(if case_sensitive {
                                        theme.colors.accent
                                    } else {
                                        theme.colors.text_muted
                                    })
                                    .border_1()
                                    .border_color(if case_sensitive {
                                        theme.colors.accent
                                    } else {
                                        theme.colors.border.opacity(0.5)
                                    })
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_search_case_sensitive(cx);
                                    }))
                                    .child("Aa"),
                            )
                            // Regex toggle
                            .child(
                                div()
                                    .id("search-regex")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .bg(if regex_enabled {
                                        theme.colors.accent.opacity(0.2)
                                    } else {
                                        theme.colors.surface
                                    })
                                    .text_color(if regex_enabled {
                                        theme.colors.accent
                                    } else {
                                        theme.colors.text_muted
                                    })
                                    .border_1()
                                    .border_color(if regex_enabled {
                                        theme.colors.accent
                                    } else {
                                        theme.colors.border.opacity(0.5)
                                    })
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_search_regex(cx);
                                    }))
                                    .child(".*"),
                            )
                            // Role filter dropdown
                            .child(
                                div()
                                    .id("search-role-filter")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_xs()
                                    .bg(if role_filter != MessageFilter::All {
                                        theme.colors.accent.opacity(0.2)
                                    } else {
                                        theme.colors.surface
                                    })
                                    .text_color(if role_filter != MessageFilter::All {
                                        theme.colors.accent
                                    } else {
                                        theme.colors.text_muted
                                    })
                                    .border_1()
                                    .border_color(if role_filter != MessageFilter::All {
                                        theme.colors.accent
                                    } else {
                                        theme.colors.border.opacity(0.5)
                                    })
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.cycle_search_role_filter(cx);
                                    }))
                                    .child(role_filter.icon())
                                    .child(role_filter.label()),
                            ),
                    )
                    // Results counter
                    .child(
                        div()
                            .text_xs()
                            .text_color(if result_count > 0 {
                                theme.colors.text
                            } else {
                                theme.colors.text_muted
                            })
                            .child(if result_count == 0 && !query.is_empty() {
                                "No results".to_string()
                            } else if result_count > 0 {
                                format!("{}/{}", current_index, result_count)
                            } else {
                                "".to_string()
                            }),
                    )
                    // Navigation buttons
                    .when(result_count > 0, |d| {
                        d.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                // Previous button
                                .child(
                                    div()
                                        .id("search-prev")
                                        .flex()
                                        .items_center()
                                        .gap(px(2.0))
                                        .px_2()
                                        .py_1()
                                        .rounded_md()
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .hover(|s| {
                                            s.bg(theme.colors.surface_hover)
                                                .text_color(theme.colors.text)
                                        })
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.prev_search_result(cx);
                                        }))
                                        .child("▲"),
                                )
                                // Next button
                                .child(
                                    div()
                                        .id("search-next")
                                        .flex()
                                        .items_center()
                                        .gap(px(2.0))
                                        .px_2()
                                        .py_1()
                                        .rounded_md()
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .hover(|s| {
                                            s.bg(theme.colors.surface_hover)
                                                .text_color(theme.colors.text)
                                        })
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.next_search_result(cx);
                                        }))
                                        .child("▼"),
                                )
                                // Jump to message button
                                .child(
                                    div()
                                        .id("search-jump")
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py_1()
                                        .ml_1()
                                        .rounded_md()
                                        .cursor_pointer()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .bg(theme.colors.accent.opacity(0.1))
                                        .text_color(theme.colors.accent)
                                        .border_1()
                                        .border_color(theme.colors.accent.opacity(0.3))
                                        .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.scroll_to_search_result(cx);
                                        }))
                                        .child("↵")
                                        .child("Jump"),
                                ),
                        )
                    })
                    // Close button
                    .child(
                        div()
                            .id("search-close")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .hover(|s| {
                                s.bg(theme.colors.surface_hover)
                                    .text_color(theme.colors.text)
                            })
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_search(cx);
                            }))
                            .child("✕"),
                    ),
            )
            // Search results preview (show first 3 results when there are matches)
            .when(result_count > 0, |d| {
                d.child(
                    div()
                        .id("search-results-preview")
                        .flex()
                        .flex_col()
                        .px_4()
                        .py_1()
                        .gap_1()
                        .bg(theme.colors.background.opacity(0.5))
                        .max_h(px(100.0))
                        .overflow_y_scroll()
                        .children(self.search.results.iter().take(5).enumerate().map(
                            |(idx, result)| {
                                let is_current = idx == self.search.current_result;
                                let result_idx = idx;
                                div()
                                    .id(ElementId::Name(format!("search-result-{}", idx).into()))
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(if is_current {
                                        theme.colors.accent.opacity(0.1)
                                    } else {
                                        theme.colors.surface.opacity(0.5)
                                    })
                                    .border_l_2()
                                    .border_color(if is_current {
                                        theme.colors.accent
                                    } else {
                                        theme.colors.border.opacity(0.3)
                                    })
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.jump_to_search_result(result_idx, cx);
                                    }))
                                    // Role indicator
                                    .child(
                                        div()
                                            .text_xs()
                                            .px_1()
                                            .rounded(px(2.0))
                                            .bg(theme.colors.surface)
                                            .text_color(theme.colors.text_muted)
                                            .child(match result.role {
                                                MessageRole::User => "You",
                                                MessageRole::Assistant => "Claude",
                                                MessageRole::ToolUse => "Tool",
                                                MessageRole::ToolResult => "Result",
                                                _ => "...",
                                            }),
                                    )
                                    // Snippet
                                    .child(
                                        div()
                                            .flex_1()
                                            .text_xs()
                                            .text_color(theme.colors.text)
                                            .overflow_hidden()
                                            .child(result.snippet.clone()),
                                    )
                            },
                        )),
                )
            })
    }
}
