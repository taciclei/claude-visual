//! Footer rendering for server configuration editor

use gpui::prelude::*;
use gpui::*;

use crate::app::theme::Theme;
use crate::ui::mcp::server_config::core::ServerConfigEditor;

impl ServerConfigEditor {
    pub(crate) fn render_footer(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let is_new = self.config.is_new;
        let error_color = theme.colors.error;
        let text_muted = theme.colors.text_muted;
        let surface_hover = theme.colors.surface_hover;
        let background = theme.colors.background;
        let accent = theme.colors.accent;
        let border_color = theme.colors.border;

        let on_delete = cx.listener(|this, _, _window, cx| {
            this.delete(cx);
        });

        let on_cancel = cx.listener(|this, _, _window, cx| {
            this.cancel(cx);
        });

        let on_save = cx.listener(|this, _, _window, cx| {
            this.save(cx);
        });

        div()
            .w_full()
            .px_4()
            .py_3()
            .flex()
            .items_center()
            .justify_between()
            .border_t_1()
            .border_color(border_color)
            // Delete button (only for existing servers)
            .child(div().when(!is_new, |d| {
                d.child(
                    div()
                        .id("delete-server-button")
                        .px_4()
                        .py_2()
                        .text_sm()
                        .text_color(error_color)
                        .cursor_pointer()
                        .rounded_md()
                        .hover(|s| s.bg(error_color.opacity(0.1)))
                        .on_click(on_delete)
                        .child("Delete Server"),
                )
            }))
            // Action buttons
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Cancel button
                    .child(
                        div()
                            .id("config-cancel-button")
                            .px_4()
                            .py_2()
                            .text_sm()
                            .text_color(text_muted)
                            .cursor_pointer()
                            .rounded_md()
                            .hover(|s| s.bg(surface_hover))
                            .on_click(on_cancel)
                            .child("Cancel"),
                    )
                    // Save button
                    .child(
                        div()
                            .id("config-save-button")
                            .px_4()
                            .py_2()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(background)
                            .bg(accent)
                            .cursor_pointer()
                            .rounded_md()
                            .hover(|s| s.opacity(0.9))
                            .on_click(on_save)
                            .child(if is_new { "Add Server" } else { "Save Changes" }),
                    ),
            )
    }
}
