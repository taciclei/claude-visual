//! Conversation actions render functions

use crate::ui::chat::view::core::ChatView;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    /// Renders conversation action buttons (copy, export, clear)
    pub fn render_conversation_actions(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            // Copy conversation button
            .child(
                div()
                    .id("copy-conversation")
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .hover(|s| {
                        s.bg(theme.colors.surface_hover)
                            .text_color(theme.colors.text)
                    })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.copy_conversation_to_clipboard(cx);
                    }))
                    .child("Copy")
                    .child(
                        div()
                            .text_color(theme.colors.text_muted.opacity(0.5))
                            .ml_1()
                            .child("⇧⌘C"),
                    ),
            )
            // Export conversation button
            .child(
                div()
                    .id("export-conversation")
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .hover(|s| {
                        s.bg(theme.colors.surface_hover)
                            .text_color(theme.colors.text)
                    })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.request_export(cx);
                    }))
                    .child("Export")
                    .child(
                        div()
                            .text_color(theme.colors.text_muted.opacity(0.5))
                            .ml_1()
                            .child("⌘E"),
                    ),
            )
            // Clear conversation button
            .child(
                div()
                    .id("clear-conversation")
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .hover(|s| {
                        s.bg(theme.colors.error.opacity(0.1))
                            .text_color(theme.colors.error)
                    })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.request_clear_conversation(cx);
                    }))
                    .child("Clear"),
            )
    }
}
