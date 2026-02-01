//! Export panel actions component

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use crate::ui::chat::view::core::ChatView;
use crate::ui::chat::view::types::NotificationType;

impl ChatView {
    pub(super) fn render_export_actions(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let has_messages = !self.messages.is_empty();
        let current_format = self.export.format;

        let text_color = theme.colors.text;
        let surface_color = theme.colors.surface;

        let on_copy = cx.listener(|this, _, _window, cx| {
            if this.has_messages() {
                let content = this.export_with_format();
                cx.write_to_clipboard(ClipboardItem::new_string(content));
                this.show_notification(
                    format!("Copied as {}", this.export.format.display_name()),
                    NotificationType::Success,
                    cx
                );
                this.toggle_export_panel(cx);
            }
        });

        let on_export = cx.listener(|this, _, _window, cx| {
            if this.has_messages() {
                let content = this.export_with_format();
                cx.write_to_clipboard(ClipboardItem::new_string(content));
                this.show_notification(
                    format!("Exported as {} (copied to clipboard)", this.export.format.display_name()),
                    NotificationType::Success,
                    cx
                );
                this.toggle_export_panel(cx);
            }
        });

        div()
            .px_4()
            .py_3()
            .border_t_1()
            .border_color(theme.colors.border)
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(format!("Output: conversation.{}", current_format.extension()))
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    // Copy to clipboard button
                    .child(
                        div()
                            .id("export-copy-btn")
                            .px_3()
                            .py_1p5()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(theme.colors.surface_hover)
                            .hover(|s| s.bg(theme.colors.border))
                            .when(!has_messages, |d| {
                                d.opacity(0.5).cursor_not_allowed()
                            })
                            .on_click(on_copy)
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(text_color)
                                    .child("Copy")
                            )
                    )
                    // Export button
                    .child(
                        div()
                            .id("export-save-btn")
                            .px_3()
                            .py_1p5()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(theme.colors.accent)
                            .hover(|s| s.bg(theme.colors.accent_hover))
                            .when(!has_messages, |d| {
                                d.opacity(0.5).cursor_not_allowed()
                            })
                            .on_click(on_export)
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(surface_color)
                                    .child("Export")
                            )
                    )
            )
    }
}
