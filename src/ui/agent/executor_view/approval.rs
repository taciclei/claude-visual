//! Approval prompt rendering

use gpui::*;
use gpui::prelude::*;
use super::{ExecutorView, ExecutorViewEvent};

impl ExecutorView {
    /// Render approval prompt
    pub(super) fn render_approval_prompt(&self, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(description) = &self.pending_approval {
            // Extract listeners before div chains
            let on_approve = cx.listener(|_this, _, _window, cx| {
                cx.emit(ExecutorViewEvent::Approve);
            });

            let on_reject = cx.listener(|_this, _, _window, cx| {
                cx.emit(ExecutorViewEvent::Reject);
            });

            // Copy theme colors for move closures
            let warning_color = self.theme.colors.warning;
            let text_color = self.theme.colors.text;
            let success_color = self.theme.colors.success;
            let error_color = self.theme.colors.error;
            let bg_color = self.theme.colors.background;

            let description_text = description.clone();

            div()
                .flex()
                .flex_col()
                .gap_2()
                .p_3()
                .bg(warning_color.opacity(0.1))
                .border_1()
                .border_color(warning_color)
                .rounded_md()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .text_lg()
                                .child("🔐")
                        )
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(warning_color)
                                .child("Approval Required")
                        )
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(text_color)
                        .child(description_text)
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .mt_2()
                        .child(
                            div()
                                .id("approve-btn")
                                .px_3()
                                .py_1()
                                .bg(success_color)
                                .text_color(bg_color)
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .rounded_md()
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.9))
                                .on_click(on_approve)
                                .child("✓ Approve")
                        )
                        .child(
                            div()
                                .id("reject-btn")
                                .px_3()
                                .py_1()
                                .bg(error_color)
                                .text_color(bg_color)
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .rounded_md()
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.9))
                                .on_click(on_reject)
                                .child("✕ Reject")
                        )
                )
        } else {
            div()
        }
    }
}
