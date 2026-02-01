//! Error retry bar panel render functions

use gpui::*;
use gpui::prelude::*;

use super::super::core::ChatView;
use super::super::types::{ErrorCategory, NotificationType};

impl ChatView {
    /// Handle error suggestion action
    pub fn handle_error_suggestion(&mut self, action: &str, cx: &mut Context<Self>) {
        match action {
            "retry" => self.retry_last_request(cx),
            "wait_retry" => {
                // Show notification and schedule retry
                self.show_notification("Waiting before retry...", NotificationType::Info, cx);
                self.retry_last_request(cx);
            }
            "check_connection" => {
                self.send_slash_command("/doctor", cx);
            }
            "new_conversation" => {
                self.clear_conversation(cx);
            }
            "show_details" => {
                // Toggle expanded error details
                if let Some(ref error) = self.last_error {
                    self.show_notification(&format!("Error details: {}", error.message), NotificationType::Error, cx);
                }
            }
            cmd if cmd.starts_with('/') => {
                self.send_slash_command(cmd, cx);
            }
            _ => {}
        }
    }

    /// Get error title based on category
    pub fn get_error_title(category: ErrorCategory) -> &'static str {
        match category {
            ErrorCategory::Network => "Connection Issue",
            ErrorCategory::RateLimit => "Rate Limited",
            ErrorCategory::ContextOverflow => "Context Too Large",
            ErrorCategory::Auth => "Authentication Error",
            ErrorCategory::ToolError => "Tool Execution Failed",
            ErrorCategory::General => "Operation Failed",
        }
    }

    /// Render error retry bar with smart suggestions
    pub fn render_error_retry_bar(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> Div {
        let error = self.last_error.as_ref().unwrap();
        let suggestions = error.category.suggestions();
        let skill_suggestions = error.category.skill_suggestions();
        let category_icon = error.category.icon();
        let error_title = Self::get_error_title(error.category);
        let tip = error.category.tip();

        div()
            .w_full()
            .px_4()
            .py_3()
            .border_t_1()
            .border_color(theme.colors.error.opacity(0.3))
            .bg(theme.colors.error.opacity(0.05))
            .flex()
            .flex_col()
            .gap_2()
            // Main error row
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    // Error icon (category-specific)
                    .child(
                        div()
                            .size(px(32.0))
                            .rounded_md()
                            .bg(theme.colors.error.opacity(0.1))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_base()
                            .child(category_icon)
                    )
                    // Error message
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.colors.error)
                                    .child(error_title)
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .max_w(px(500.0))
                                    .overflow_x_hidden()
                                    .child(error.message.clone())
                            )
                    )
                    // Dismiss button
                    .child(
                        div()
                            .id("dismiss-error")
                            .size(px(24.0))
                            .rounded_md()
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.clear_error(cx);
                            }))
                            .child("×")
                    )
            )
            // Quick actions row
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .pl(px(44.0)) // Align with error text
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Quick:")
                    )
                    .children(suggestions.iter().enumerate().map(|(idx, (icon, label, action))| {
                        let action_str = action.to_string();
                        let is_primary = idx == 0;

                        div()
                            .id(SharedString::from(format!("error-suggestion-{}", idx)))
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .text_xs()
                            .when(is_primary, |d| {
                                d.bg(theme.colors.warning.opacity(0.15))
                                    .border_1()
                                    .border_color(theme.colors.warning.opacity(0.3))
                                    .text_color(theme.colors.warning)
                                    .font_weight(FontWeight::MEDIUM)
                            })
                            .when(!is_primary, |d| {
                                d.bg(theme.colors.surface)
                                    .border_1()
                                    .border_color(theme.colors.border)
                                    .text_color(theme.colors.text_muted)
                            })
                            .hover(|s| {
                                s.bg(theme.colors.surface_hover)
                                    .border_color(theme.colors.accent.opacity(0.5))
                            })
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.handle_error_suggestion(&action_str, cx);
                            }))
                            .child(*icon)
                            .child(*label)
                    }))
            )
            // Extended skill suggestions row
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .pl(px(44.0))
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Skills:")
                    )
                    .children(skill_suggestions.iter().take(4).enumerate().map(|(idx, (icon, label, action, desc))| {
                        let action_str = action.to_string();
                        let desc_str = desc.to_string();

                        div()
                            .id(SharedString::from(format!("error-skill-{}", idx)))
                            .flex()
                            .flex_col()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(theme.colors.accent.opacity(0.05))
                            .border_1()
                            .border_color(theme.colors.accent.opacity(0.15))
                            .hover(|s| {
                                s.bg(theme.colors.accent.opacity(0.15))
                                    .border_color(theme.colors.accent.opacity(0.3))
                            })
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.handle_error_suggestion(&action_str, cx);
                            }))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .text_xs()
                                    .text_color(theme.colors.accent)
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(*icon)
                                    .child(*label)
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted.opacity(0.7))
                                    .child(desc_str)
                            )
                    }))
            )
            // Tip row
            .child(
                div()
                    .pl(px(44.0))
                    .flex()
                    .items_center()
                    .gap_1()
                    .text_xs()
                    .text_color(theme.colors.info.opacity(0.8))
                    .child("💡")
                    .child(tip)
            )
    }

    /// Render a detail row (label: value)
    pub fn render_detail_row(&self, label: &str, value: &str, theme: &crate::app::theme::Theme) -> Div {
        div()
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(label.to_string())
            )
            .child(
                div()
                    .text_xs()
                    .font_family("monospace")
                    .text_color(theme.colors.text)
                    .child(value.to_string())
            )
    }

    /// Render a detail row with a copy button
    pub fn render_detail_row_with_copy(&self, label: &str, value: &str, id_suffix: &str, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> Div {
        let value_to_copy = value.to_string();
        div()
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(label.to_string())
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .font_family("monospace")
                            .text_color(theme.colors.text)
                            .max_w(px(200.0))
                            .overflow_hidden()
                            .child(value.to_string())
                    )
                    .child(
                        div()
                            .id(ElementId::Name(format!("copy-{}", id_suffix).into()))
                            .w(px(18.0))
                            .h(px(18.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_sm()
                            .cursor_pointer()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                            .on_click(cx.listener(move |_this, _, _window, cx| {
                                cx.write_to_clipboard(gpui::ClipboardItem::new_string(value_to_copy.clone()));
                            }))
                            .child("📋")
                    )
            )
    }
}
