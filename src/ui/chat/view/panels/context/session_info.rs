//! Context panel session info component

use gpui::*;
use gpui::prelude::*;

use super::super::super::core::ChatView;
use crate::app::theme::Theme;
use crate::claude::message::SessionInfo;

impl ChatView {
    pub(super) fn render_session_info(&self, theme: &Theme, info: &SessionInfo) -> impl IntoElement {
        div()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .mb_2()
                    .child(div().text_sm().child("📋"))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text_muted)
                            .child("Session Info")
                    )
            )
            .child(
                div()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(theme.colors.background)
                    .border_1()
                    .border_color(theme.colors.border.opacity(0.5))
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("Model")
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.colors.text)
                                    .child(info.model.clone())
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("Working Directory")
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.colors.text)
                                    .max_w(px(250.0))
                                    .overflow_hidden()
                                    .child(info.cwd.clone())
                            )
                    )
                    .when(!info.session_id.is_empty(), |d| {
                        d.child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child("Session ID")
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_family("monospace")
                                        .text_color(theme.colors.text)
                                        .child(info.session_id.chars().take(16).collect::<String>() + "...")
                                )
                        )
                    })
            )
    }
}
