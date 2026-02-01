//! Header rendering for history sidebar

use gpui::*;
use gpui::prelude::*;

use super::super::core::HistorySidebar;
use super::super::types::{DisplayMode, HistorySidebarEvent};

impl HistorySidebar {
    pub(super) fn render_header(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let display_mode = self.display_mode;
        let search_query = self.search_query.clone();

        div()
            .flex_shrink_0()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(theme.colors.border)
            .flex()
            .flex_col()
            .gap_2()
            .child(self.render_title_row(display_mode, cx))
            .child(self.render_search_input(search_query, display_mode, _window, cx))
            .child(self.render_quick_actions(cx))
            .when(self.show_filters, |d| {
                d.child(self.render_filter_panel(cx))
            })
    }

    fn render_quick_actions(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let accent = theme.colors.accent;
        let info = theme.colors.info;
        let success = theme.colors.success;
        let warning = theme.colors.warning;

        div()
            .flex()
            .flex_wrap()
            .gap_1()
            .py_1()
            // Resume session button
            .child(
                div()
                    .id("history-action-resume")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(accent.opacity(0.15))
                    .border_1()
                    .border_color(accent.opacity(0.3))
                    .text_xs()
                    .text_color(accent)
                    .hover(move |s| s.bg(accent.opacity(0.25)).border_color(accent.opacity(0.5)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(HistorySidebarEvent::SendSkillCommand("/resume".to_string()));
                    }))
                    .child("▶ Resume")
            )
            // Compact context button
            .child(
                div()
                    .id("history-action-compact")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(info.opacity(0.15))
                    .border_1()
                    .border_color(info.opacity(0.3))
                    .text_xs()
                    .text_color(info)
                    .hover(move |s| s.bg(info.opacity(0.25)).border_color(info.opacity(0.5)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(HistorySidebarEvent::SendSkillCommand("/compact".to_string()));
                    }))
                    .child("🗜️ Compact")
            )
            // Usage stats button
            .child(
                div()
                    .id("history-action-usage")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(success.opacity(0.15))
                    .border_1()
                    .border_color(success.opacity(0.3))
                    .text_xs()
                    .text_color(success)
                    .hover(move |s| s.bg(success.opacity(0.25)).border_color(success.opacity(0.5)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(HistorySidebarEvent::SendSkillCommand("/usage".to_string()));
                    }))
                    .child("📊 Usage")
            )
            // Memory button
            .child(
                div()
                    .id("history-action-memory")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(warning.opacity(0.15))
                    .border_1()
                    .border_color(warning.opacity(0.3))
                    .text_xs()
                    .text_color(warning)
                    .hover(move |s| s.bg(warning.opacity(0.25)).border_color(warning.opacity(0.5)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(HistorySidebarEvent::SendSkillCommand("/claude-memory".to_string()));
                    }))
                    .child("📝 Memory")
            )
    }

    fn render_title_row(
        &mut self,
        display_mode: DisplayMode,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let filter_active = self.search_filter.is_active();
        let show_filters = self.show_filters;

        div()
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.text_muted)
                    .child(if display_mode == DisplayMode::Search {
                        format!("SEARCH ({})", self.search_results.len())
                    } else {
                        "HISTORY".to_string()
                    }),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child({
                        div()
                            .id("toggle-filters")
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .text_xs()
                            .text_color(if filter_active {
                                theme.colors.accent
                            } else {
                                theme.colors.text_muted
                            })
                            .bg(if show_filters {
                                theme.colors.surface_hover
                            } else {
                                theme.colors.surface
                            })
                            .hover(|style| {
                                style
                                    .bg(theme.colors.surface_hover)
                                    .text_color(theme.colors.text)
                            })
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_filters(cx);
                            }))
                            .child(if filter_active { "⚙ Filter" } else { "Filter" })
                    })
                    .child(
                        div()
                            .id("refresh-history")
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .hover(|style| {
                                style
                                    .bg(theme.colors.surface_hover)
                                    .text_color(theme.colors.text)
                            })
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.refresh(cx);
                            }))
                            .child("Refresh"),
                    ),
            )
    }

    fn render_search_input(
        &mut self,
        search_query: String,
        display_mode: DisplayMode,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let is_search_focused = self.search_focus_handle.is_focused(_window);

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .id("search-input-container")
                    .track_focus(&self.search_focus_handle)
                    .flex_1()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .border_1()
                    .border_color(if is_search_focused {
                        theme.colors.accent
                    } else {
                        theme.colors.border
                    })
                    .bg(theme.colors.background)
                    .flex()
                    .items_center()
                    .gap_2()
                    .cursor_text()
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.search_focus_handle.focus(window);
                        cx.notify();
                    }))
                    .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                        this.handle_search_key_down(event, window, cx);
                    }))
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("🔍"),
                    )
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(if search_query.is_empty() {
                                theme.colors.text_muted
                            } else {
                                theme.colors.text
                            })
                            .child(if search_query.is_empty() {
                                "Search messages...".to_string()
                            } else {
                                search_query.clone()
                            }),
                    ),
            )
            .when(display_mode == DisplayMode::Search, |d| {
                d.child(
                    div()
                        .id("clear-search")
                        .px_2()
                        .py_1()
                        .rounded_sm()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .hover(|style| {
                            style
                                .bg(theme.colors.surface_hover)
                                .text_color(theme.colors.text)
                        })
                        .cursor_pointer()
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.clear_search(cx);
                        }))
                        .child("Clear"),
                )
            })
    }
}
