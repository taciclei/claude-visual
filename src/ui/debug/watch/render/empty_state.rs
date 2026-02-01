//! Empty state rendering

use gpui::prelude::*;
use gpui::*;

use super::super::core::WatchView;
use super::super::events::WatchViewEvent;
use crate::app::theme::Theme;

impl WatchView {
    pub fn render_empty_state(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let text_muted = theme.colors.text_muted;
        let accent = theme.colors.accent;
        let info = theme.colors.info;

        let on_add = cx.listener(|this, _, _window, cx| {
            this.start_adding(cx);
        });

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .py_4()
            .gap_3()
            .child(
                div()
                    .size(px(40.0))
                    .rounded_full()
                    .bg(text_muted.opacity(0.1))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(div().text_lg().child("👁")),
            )
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.colors.text)
                    .child("No watch expressions"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(text_muted)
                    .text_center()
                    .max_w(px(200.0))
                    .child("Add expressions to monitor values during debugging"),
            )
            .child(
                div()
                    .id("add-watch-empty")
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .text_xs()
                    .text_color(accent)
                    .bg(accent.opacity(0.1))
                    .cursor_pointer()
                    .hover(move |s| s.bg(accent.opacity(0.2)))
                    .on_click(on_add)
                    .child("+ Add Expression"),
            )
            // Skill suggestions
            .child(
                div()
                    .pt_2()
                    .flex()
                    .flex_wrap()
                    .justify_center()
                    .gap_2()
                    // Debug skill
                    .child(
                        div()
                            .id("watch-empty-debug")
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
                                cx.emit(WatchViewEvent::SendSkillCommand("/debug".to_string()));
                            }))
                            .child("🐛 /debug"),
                    )
                    // Explore skill
                    .child(
                        div()
                            .id("watch-empty-explore")
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
                                cx.emit(WatchViewEvent::SendSkillCommand("/explore".to_string()));
                            }))
                            .child("🔍 /explore"),
                    ),
            )
    }
}
