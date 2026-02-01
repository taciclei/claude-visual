//! Context panel tools section component

use gpui::*;
use gpui::prelude::*;

use super::super::super::core::ChatView;
use crate::app::theme::Theme;
use crate::claude::message::SessionInfo;

impl ChatView {
    pub(super) fn render_tools_section(&self, theme: &Theme, info: &SessionInfo) -> impl IntoElement {
        div()
            .mb_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .mb_2()
                    .child(div().text_sm().child("🔧"))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text_muted)
                            .child(format!("Available Tools ({})", info.tools.len()))
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_1()
                    .children(info.tools.iter().take(20).map(|tool| {
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded_md()
                            .bg(theme.colors.accent.opacity(0.1))
                            .text_xs()
                            .text_color(theme.colors.accent)
                            .child(tool.clone())
                    }))
                    .when(info.tools.len() > 20, |d| {
                        d.child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded_md()
                                .bg(theme.colors.text_muted.opacity(0.1))
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child(format!("+{} more", info.tools.len() - 20))
                        )
                    })
            )
    }
}
