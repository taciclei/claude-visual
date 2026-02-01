//! Pinned messages panel render functions

use gpui::prelude::*;
use gpui::*;

use super::super::core::ChatView;
use crate::claude::message::MessageRole;

impl ChatView {
    pub fn render_pinned_panel(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let pinned = self.get_pinned_messages();
        let has_pinned = !pinned.is_empty();

        div()
            .id("pinned-panel-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_pinned_panel(cx);
            }))
            .child(
                div()
                    .id("pinned-panel")
                    .w(px(500.0))
                    .max_h(px(450.0))
                    .bg(theme.colors.surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_click(|_, _, _| {})
                    // Header
                    .child(
                        div()
                            .px_4()
                            .py_3()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().text_base().child("📌"))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child("Pinned Messages"),
                                    )
                                    .when(has_pinned, |d| {
                                        d.child(
                                            div()
                                                .px_2()
                                                .py_0p5()
                                                .rounded_full()
                                                .bg(theme.colors.accent.opacity(0.15))
                                                .text_xs()
                                                .text_color(theme.colors.accent)
                                                .child(format!("{}", pinned.len())),
                                        )
                                    }),
                            )
                            .child(
                                div()
                                    .id("close-pinned-panel")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_pinned_panel(cx);
                                    }))
                                    .child("×"),
                            ),
                    )
                    // Pinned messages list
                    .child(
                        div()
                            .id("pinned-list")
                            .flex_1()
                            .overflow_y_scroll()
                            .when(!has_pinned, |d| {
                                d.child(
                                    div()
                                        .p_8()
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .gap_2()
                                        .child(div().text_3xl().child("📌"))
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(theme.colors.text_muted)
                                                .child("No pinned messages"),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child("Use ⌥P to pin messages for quick access"),
                                        ),
                                )
                            })
                            .when(has_pinned, |d| {
                                d.children(pinned.iter().map(|(idx, msg)| {
                                    let msg_idx = *idx;
                                    let preview = msg.content.chars().take(120).collect::<String>();
                                    let role_icon = match msg.role {
                                        MessageRole::User => "👤",
                                        MessageRole::Assistant => "🤖",
                                        MessageRole::ToolUse => "🔧",
                                        MessageRole::ToolResult => "📋",
                                        _ => "💬",
                                    };
                                    div()
                                        .id(ElementId::Name(format!("pinned-msg-{}", idx).into()))
                                        .px_4()
                                        .py_3()
                                        .border_b_1()
                                        .border_color(theme.colors.border.opacity(0.5))
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.colors.surface_hover))
                                        .on_click(cx.listener(move |this, _, _window, cx| {
                                            // Close panel and scroll to message
                                            this.toggle_pinned_panel(cx);
                                            this.select_message(Some(msg_idx), cx);
                                        }))
                                        .child(
                                            div()
                                                .flex()
                                                .items_start()
                                                .gap_2()
                                                .child(div().text_sm().child(role_icon))
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_2()
                                                                .mb_1()
                                                                .child(
                                                                    div()
                                                                        .text_xs()
                                                                        .font_weight(
                                                                            FontWeight::MEDIUM,
                                                                        )
                                                                        .text_color(
                                                                            theme.colors.text_muted,
                                                                        )
                                                                        .child(format!(
                                                                            "Message #{}",
                                                                            idx + 1
                                                                        )),
                                                                ),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .text_color(theme.colors.text)
                                                                .max_w(px(380.0))
                                                                .overflow_hidden()
                                                                .child(if preview.len() >= 120 {
                                                                    format!("{}...", preview)
                                                                } else {
                                                                    preview
                                                                }),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .id(ElementId::Name(
                                                            format!("unpin-msg-{}", idx).into(),
                                                        ))
                                                        .px_1()
                                                        .rounded_sm()
                                                        .cursor_pointer()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .hover(|s| s.text_color(theme.colors.error))
                                                        .on_click(cx.listener(
                                                            move |this, _, _window, cx| {
                                                                this.toggle_pin(msg_idx, cx);
                                                            },
                                                        ))
                                                        .child("×"),
                                                ),
                                        )
                                }))
                            }),
                    ),
            )
    }
}
