//! Error message rendering

use gpui::prelude::*;
use gpui::*;

use super::super::types::MessageViewEvent;
use super::super::utils::categorize_error;
use super::super::view::MessageView;
use crate::app::theme::Theme;
use crate::ui::chat::view::types::ErrorCategory;

impl MessageView {
    pub(in crate::ui::chat::message) fn render_error_content(
        &self,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        let content = self.message.content.clone();

        // Categorize the error
        let (error_icon, error_title, is_retryable) = categorize_error(&content);

        // Get error category for smart suggestions
        let error_category = ErrorCategory::from_message(&content);
        let skill_suggestions = error_category.skill_suggestions();
        let tip = error_category.tip();

        div()
            .px_3()
            .py_3()
            .bg(theme.colors.error.opacity(0.05))
            .border_l_2()
            .border_color(theme.colors.error.opacity(0.5))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    // Error header with icon and title
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_base().child(error_icon))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.colors.error)
                                    .child(error_title),
                            ),
                    )
                    // Error message
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .font_family("JetBrains Mono")
                            .child(content),
                    )
                    // Action buttons
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .mt_1()
                            // Retry button (if retryable)
                            .when(is_retryable, |d| {
                                d.child(
                                    div()
                                        .id("error-retry-button")
                                        .px_3()
                                        .py_1()
                                        .rounded_md()
                                        .bg(theme.colors.error.opacity(0.1))
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.colors.error)
                                        .hover(|s| s.bg(theme.colors.error.opacity(0.2)))
                                        .cursor_pointer()
                                        .on_click(cx.listener(|_this, _, _window, cx| {
                                            cx.emit(MessageViewEvent::RetryFromHere);
                                        }))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1()
                                                .child("🔄")
                                                .child("Retry"),
                                        ),
                                )
                            })
                            // Copy error button
                            .child(
                                div()
                                    .id("error-copy-button")
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .bg(theme.colors.surface)
                                    .border_1()
                                    .border_color(theme.colors.border)
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .cursor_pointer()
                                    .on_click(cx.listener(|_this, _, _window, cx| {
                                        cx.emit(MessageViewEvent::Copy);
                                    }))
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .child("📋")
                                            .child("Copy Error"),
                                    ),
                            ),
                    )
                    // Skill suggestions for error recovery
                    .child(
                        div()
                            .mt_2()
                            .pt_2()
                            .border_t_1()
                            .border_color(theme.colors.border.opacity(0.3))
                            .flex()
                            .flex_col()
                            .gap_2()
                            // Suggestions header
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("Recovery suggestions:"),
                            )
                            // Skill buttons
                            .child(div().flex().flex_wrap().gap_2().children(
                                skill_suggestions.into_iter().enumerate().map(
                                    |(idx, (icon, label, cmd, desc))| {
                                        let cmd_str = cmd.to_string();
                                        div()
                                            .id(SharedString::from(format!("error-skill-{}", idx)))
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .px_2()
                                            .py_1()
                                            .rounded_md()
                                            .bg(theme.colors.accent.opacity(0.1))
                                            .border_1()
                                            .border_color(theme.colors.accent.opacity(0.2))
                                            .text_xs()
                                            .text_color(theme.colors.accent)
                                            .cursor_pointer()
                                            .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
                                            .on_click(cx.listener(move |_this, _, _window, cx| {
                                                cx.emit(MessageViewEvent::ExecuteSkill(
                                                    cmd_str.clone(),
                                                ));
                                            }))
                                            .child(icon)
                                            .child(
                                                div().flex().flex_col().child(label).child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .child(desc),
                                                ),
                                            )
                                    },
                                ),
                            ))
                            // Pro tip
                            .child(
                                div()
                                    .mt_1()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .bg(theme.colors.info.opacity(0.1))
                                    .text_xs()
                                    .text_color(theme.colors.info)
                                    .child(tip),
                            ),
                    ),
            )
    }
}
