//! Header rendering for server configuration editor

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use crate::ui::mcp::server_config::core::ServerConfigEditor;

impl ServerConfigEditor {
    pub(crate) fn render_header(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let is_new = self.config.is_new;
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let border_color = theme.colors.border;

        let on_close = cx.listener(|this, _, _window, cx| {
            this.cancel(cx);
        });

        div()
            .w_full()
            .px_4()
            .py_3()
            .flex()
            .items_center()
            .justify_between()
            .border_b_1()
            .border_color(border_color)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text_color)
                            .child(if is_new {
                                "Add MCP Server"
                            } else {
                                "Configure Server"
                            }),
                    ),
            )
            .child(
                div()
                    .id("config-close-button")
                    .cursor_pointer()
                    .text_color(text_muted)
                    .hover(|s| s.text_color(text_color))
                    .on_click(on_close)
                    .child("✕"),
            )
    }
}
