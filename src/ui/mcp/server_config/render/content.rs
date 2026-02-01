//! Content rendering for server configuration editor

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use crate::ui::mcp::server_config::core::ServerConfigEditor;
use crate::ui::mcp::server_config::types::EditingField;

impl ServerConfigEditor {
    pub(crate) fn render_content(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let success_color = theme.colors.success;
        let surface_hover = theme.colors.surface_hover;
        let text_color = theme.colors.text;
        let error_color = theme.colors.error;

        let on_toggle_enabled = cx.listener(|this, _, _window, cx| {
            this.toggle_enabled(cx);
        });

        div()
            .flex_1()
            .id("scroll-server-config-content")
            .overflow_y_scroll()
            .px_4()
            .py_4()
            .flex()
            .flex_col()
            .gap_4()
            // Server name
            .child(self.render_field(
                "Server Name",
                EditingField::Name,
                &self.config.name,
                "e.g., filesystem",
                false,
                &theme,
                cx,
            ))
            // Command
            .child(self.render_field(
                "Command",
                EditingField::Command,
                &self.config.command,
                "e.g., npx, node, python",
                false,
                &theme,
                cx,
            ))
            // Arguments
            .child(self.render_field(
                "Arguments (one per line)",
                EditingField::Args,
                &self.config.args,
                "-y\n@modelcontextprotocol/server-filesystem\n/path/to/dir",
                true,
                &theme,
                cx,
            ))
            // Environment variables
            .child(self.render_field(
                "Environment Variables (KEY=VALUE per line)",
                EditingField::Env,
                &self.config.env,
                "API_KEY=xxx\nDEBUG=true",
                true,
                &theme,
                cx,
            ))
            // Description
            .child(self.render_field(
                "Description (optional)",
                EditingField::Description,
                &self.config.description,
                "A brief description of the server",
                false,
                &theme,
                cx,
            ))
            // Auto-approve patterns
            .child(self.render_field(
                "Auto-approve Tool Patterns (one per line, use with caution)",
                EditingField::AutoApprove,
                &self.config.auto_approve,
                "read_*\nlist_*",
                true,
                &theme,
                cx,
            ))
            // Enabled toggle
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .id("enabled-toggle")
                            .w_10()
                            .h_5()
                            .rounded_full()
                            .cursor_pointer()
                            .bg(if self.config.enabled {
                                success_color
                            } else {
                                surface_hover
                            })
                            .on_click(on_toggle_enabled)
                            .child(
                                div()
                                    .size_4()
                                    .mt(px(2.0))
                                    .when(self.config.enabled, |d| d.ml(px(22.0)))
                                    .when(!self.config.enabled, |d| d.ml(px(2.0)))
                                    .rounded_full()
                                    .bg(hsla(0.0, 0.0, 1.0, 1.0))
                                    .shadow_sm(),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_color)
                            .child("Enabled"),
                    ),
            )
            // Error message
            .when_some(self.error.clone(), |d, error| {
                d.child(
                    div()
                        .w_full()
                        .px_3()
                        .py_2()
                        .bg(error_color.opacity(0.1))
                        .border_1()
                        .border_color(error_color.opacity(0.3))
                        .rounded_md()
                        .text_sm()
                        .text_color(error_color)
                        .child(error),
                )
            })
    }
}
