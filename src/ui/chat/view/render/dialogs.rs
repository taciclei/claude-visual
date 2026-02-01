//! Dialog render functions for ChatView

use super::super::core::ChatView;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    /// Render clear conversation confirmation dialog
    pub fn render_clear_confirmation_dialog(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let message_count = self.messages.len();

        // Pre-extract listeners
        let on_cancel = cx.listener(|this, _, _window, cx| {
            this.cancel_clear_confirmation(cx);
        });
        let on_confirm = cx.listener(|this, _, _window, cx| {
            this.confirm_clear_conversation(cx);
        });

        // Copy theme colors for move closures
        let surface_hover = theme.colors.surface_hover;
        let text = theme.colors.text;
        let text_muted = theme.colors.text_muted;

        div()
            .id("clear-confirmation-dialog")
            .absolute()
            .inset_0()
            .bg(theme.colors.background.opacity(0.7))
            .flex()
            .items_center()
            .justify_center()
            // Close on backdrop click
            .on_click(cx.listener(|this, _, _window, cx| {
                this.cancel_clear_confirmation(cx);
            }))
            .child(
                div()
                    .id("clear-confirmation-content")
                    .w(px(420.0))
                    .p_5()
                    .rounded_lg()
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_xl()
                    .flex()
                    .flex_col()
                    .gap_4()
                    // Prevent clicks from closing by stopping propagation
                    .on_click(|_, _window, _cx| {})
                    // Header with icon
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .size(px(44.0))
                                    .rounded_full()
                                    .bg(theme.colors.warning.opacity(0.15))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(theme.colors.warning)
                                    .text_xl()
                                    .child("⚠"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .text_base()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child("Clear Conversation?"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.colors.text_muted)
                                            .child(format!(
                                                "This will delete {} message{}.",
                                                message_count,
                                                if message_count == 1 { "" } else { "s" }
                                            )),
                                    ),
                            ),
                    )
                    // Warning box
                    .child(
                        div()
                            .p_3()
                            .rounded_md()
                            .bg(theme.colors.error.opacity(0.08))
                            .border_1()
                            .border_color(theme.colors.error.opacity(0.2))
                            .flex()
                            .items_start()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.error)
                                    .child("⚡"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.error.opacity(0.9))
                                    .child("This action cannot be undone. Your conversation history will be permanently deleted."),
                            ),
                    )
                    // Keyboard hint
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Press Escape to cancel, Enter to confirm"),
                    )
                    // Buttons
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                div()
                                    .id("clear-cancel")
                                    .px_4()
                                    .py_2()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(text_muted)
                                    .border_1()
                                    .border_color(theme.colors.border)
                                    .hover(move |s| {
                                        s.bg(surface_hover).text_color(text)
                                    })
                                    .on_click(on_cancel)
                                    .child("Cancel"),
                            )
                            .child(
                                div()
                                    .id("clear-confirm")
                                    .px_4()
                                    .py_2()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .bg(theme.colors.error)
                                    .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                    .hover(|s| s.opacity(0.9))
                                    .on_click(on_confirm)
                                    .child("Clear All"),
                            ),
                    ),
            )
    }
}
