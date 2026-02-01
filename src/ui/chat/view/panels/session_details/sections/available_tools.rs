//! Available tools section rendering

use gpui::prelude::*;
use gpui::*;

use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Renders the available tools section
    pub(crate) fn render_available_tools_section(
        &self,
        info: &crate::claude::message::SessionInfo,
        theme: &crate::app::theme::Theme,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.text_muted)
                    .child(format!("AVAILABLE TOOLS ({})", info.tools.len())),
            )
            .child(
                div()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(theme.colors.background)
                    .flex()
                    .flex_wrap()
                    .gap_1()
                    .children(info.tools.iter().take(20).map(|tool| {
                        div()
                            .px_2()
                            .py(px(2.0))
                            .rounded_sm()
                            .bg(theme.colors.accent.opacity(0.1))
                            .text_xs()
                            .text_color(theme.colors.accent)
                            .child(tool.clone())
                    }))
                    .when(info.tools.len() > 20, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child(format!("+{} more", info.tools.len() - 20)),
                        )
                    }),
            )
    }
}
