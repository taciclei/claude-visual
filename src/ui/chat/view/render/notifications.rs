//! Notifications render functions for ChatView

use gpui::*;
use gpui::prelude::*;
use super::super::core::ChatView;
use super::super::types::{NotificationType, ChatViewEvent};

impl ChatView {
    pub fn render_notifications(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> Div {
        div()
            .absolute()
            .top(px(60.0))
            .right(px(16.0))
            .flex()
            .flex_col()
            .gap_2()
            .children(self.notifications.iter().enumerate().map(|(i, notification)| {
                let (bg_color, border_color, icon) = match notification.notification_type {
                    NotificationType::Success => (theme.colors.success.opacity(0.1), theme.colors.success, "✓"),
                    NotificationType::Info => (theme.colors.info.opacity(0.1), theme.colors.info, "ℹ"),
                    NotificationType::Warning => (theme.colors.warning.opacity(0.1), theme.colors.warning, "⚠"),
                    NotificationType::Error => (theme.colors.error.opacity(0.1), theme.colors.error, "✕"),
                };

                // Pre-extract action data to avoid closure issues
                let action_data = notification.quick_action;
                let action_btn_bg = border_color.opacity(0.15);
                let action_btn_border = border_color.opacity(0.3);
                let action_btn_hover_bg = border_color.opacity(0.25);
                let action_btn_hover_border = border_color.opacity(0.5);

                let mut result = div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .px_3()
                    .py_2()
                    .rounded_lg()
                    .bg(bg_color)
                    .border_1()
                    .border_color(border_color)
                    .shadow_md()
                    .min_w(px(200.0))
                    // Main notification row
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(border_color)
                                    .child(icon)
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .child(notification.message.clone())
                            )
                    );

                // Add quick action button if present
                if let Some((action_icon, action_label, action_cmd)) = action_data {
                    let cmd = action_cmd.to_string();
                    result = result.child(
                        div()
                            .id(SharedString::from(format!("notif-action-{}", i)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .mt_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(action_btn_bg)
                            .border_1()
                            .border_color(action_btn_border)
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(border_color)
                            .hover(move |s| s.bg(action_btn_hover_bg).border_color(action_btn_hover_border))
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                // Dismiss notification
                                if i < this.notifications.len() {
                                    this.notifications.remove(i);
                                }
                                // Execute action
                                if cmd.starts_with('/') {
                                    cx.emit(ChatViewEvent::Submit(cmd.clone()));
                                }
                                cx.notify();
                            }))
                            .child(action_icon)
                            .child(action_label)
                    );
                }

                result
            }))
    }
}
