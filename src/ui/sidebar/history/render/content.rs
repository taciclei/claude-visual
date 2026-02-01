//! Content area rendering for history sidebar

use gpui::prelude::*;
use gpui::*;

use super::super::core::HistorySidebar;
use super::super::types::{DisplayMode, HistorySidebarEvent};

impl HistorySidebar {
    pub(super) fn render_content(
        &mut self,
        conversation_items: Vec<(String, String, String, bool)>,
        search_items: Vec<(String, String, String, String, String)>,
        display_mode: DisplayMode,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let mut conversation_children = Vec::new();
        for (id, title, time, is_selected) in conversation_items {
            conversation_children.push(self.render_conversation_item(
                id,
                title,
                time,
                is_selected,
                cx,
            ));
        }

        let mut search_children = Vec::new();
        for (conv_id, conv_title, content_preview, _highlighted, time) in search_items {
            search_children.push(self.render_search_result_item(
                conv_id,
                conv_title,
                content_preview,
                time,
                cx,
            ));
        }

        let theme = self.app_state.theme.read(cx);
        let accent = theme.colors.accent;
        let info = theme.colors.info;
        let text_muted = theme.colors.text_muted;
        let text = theme.colors.text;

        let is_recent = display_mode == DisplayMode::Recent;
        let is_search = display_mode == DisplayMode::Search;
        let conversation_is_empty = conversation_children.is_empty();
        let search_is_empty = search_children.is_empty();

        div()
            .flex_1()
            .id("scroll-history")
            .overflow_y_scroll()
            .px_2()
            .py_1()
            .when(is_recent, move |d| {
                d.when(conversation_is_empty, |this| {
                    this.child(
                        div()
                            .px_3()
                            .py_6()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_3()
                            .child(div().text_2xl().child("💬"))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(text)
                                    .child("No conversations yet"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .text_center()
                                    .child("Start a new conversation to get going"),
                            )
                            // Quick skill suggestions
                            .child(
                                div()
                                    .pt_2()
                                    .flex()
                                    .flex_wrap()
                                    .justify_center()
                                    .gap_2()
                                    // APEX skill
                                    .child(
                                        div()
                                            .id("history-empty-apex")
                                            .px_2()
                                            .py_1()
                                            .rounded_md()
                                            .cursor_pointer()
                                            .bg(accent.opacity(0.15))
                                            .border_1()
                                            .border_color(accent.opacity(0.3))
                                            .text_xs()
                                            .text_color(accent)
                                            .hover(move |s| {
                                                s.bg(accent.opacity(0.25))
                                                    .border_color(accent.opacity(0.5))
                                            })
                                            .on_click(cx.listener(|_this, _, _window, cx| {
                                                cx.emit(HistorySidebarEvent::SendSkillCommand(
                                                    "/apex".to_string(),
                                                ));
                                            }))
                                            .child("⚡ /apex"),
                                    )
                                    // Explore skill
                                    .child(
                                        div()
                                            .id("history-empty-explore")
                                            .px_2()
                                            .py_1()
                                            .rounded_md()
                                            .cursor_pointer()
                                            .bg(info.opacity(0.15))
                                            .border_1()
                                            .border_color(info.opacity(0.3))
                                            .text_xs()
                                            .text_color(info)
                                            .hover(move |s| {
                                                s.bg(info.opacity(0.25))
                                                    .border_color(info.opacity(0.5))
                                            })
                                            .on_click(cx.listener(|_this, _, _window, cx| {
                                                cx.emit(HistorySidebarEvent::SendSkillCommand(
                                                    "/explore".to_string(),
                                                ));
                                            }))
                                            .child("🔍 /explore"),
                                    ),
                            ),
                    )
                })
                .children(conversation_children)
            })
            .when(is_search, move |d| {
                d.when(search_is_empty, |this| {
                    this.child(
                        div()
                            .px_3()
                            .py_4()
                            .text_sm()
                            .text_color(text_muted)
                            .text_center()
                            .child("No results found"),
                    )
                })
                .children(search_children)
            })
    }

    fn render_conversation_item(
        &mut self,
        id: String,
        title: String,
        time: String,
        is_selected: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let bg_color = if is_selected {
            theme.colors.accent
        } else {
            theme.colors.surface
        };
        let hover_color = if is_selected {
            theme.colors.accent_hover
        } else {
            theme.colors.surface_hover
        };
        let id_for_click = id.clone();
        let id_for_delete = id.clone();

        let on_click_listener = cx.listener(move |this, _, _window, cx| {
            this.select_conversation(&id_for_click, cx);
        });
        let on_delete_listener = cx.listener(move |this, _, _window, cx| {
            this.delete_conversation(&id_for_delete, cx);
        });

        div()
            .id(ElementId::Name(format!("conv-{}", id).into()))
            .px_3()
            .py_2()
            .mb_1()
            .rounded_md()
            .bg(bg_color)
            .hover(|style| style.bg(hover_color))
            .cursor_pointer()
            .on_click(on_click_listener)
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(title),
                    )
                    .child(
                        div()
                            .id(ElementId::Name(format!("delete-{}", id).into()))
                            .px_1()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .hover(|style| style.text_color(theme.colors.error))
                            .cursor_pointer()
                            .on_click(on_delete_listener)
                            .child("x"),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(time),
            )
    }

    fn render_search_result_item(
        &mut self,
        conv_id: String,
        conv_title: String,
        content_preview: String,
        time: String,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let conv_id_for_click = conv_id.clone();

        let on_click_listener = cx.listener(move |this, _, _window, cx| {
            this.select_conversation(&conv_id_for_click, cx);
            this.clear_search(cx);
        });

        div()
            .id(ElementId::Name(format!("search-{}", conv_id).into()))
            .px_3()
            .py_2()
            .mb_1()
            .rounded_md()
            .bg(theme.colors.surface)
            .hover(|style| style.bg(theme.colors.surface_hover))
            .cursor_pointer()
            .on_click(on_click_listener)
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.accent)
                    .overflow_hidden()
                    .text_ellipsis()
                    .child(conv_title),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(theme.colors.text)
                    .overflow_hidden()
                    .line_clamp(2)
                    .child(content_preview),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(time),
            )
    }
}
