//! Notification management for ChatView
//!
//! This module handles user feedback notifications (toast-style messages).

use gpui::Context;

use super::ChatView;
use crate::ui::chat::view::types::{Notification, NotificationType, get_notification_action};

impl ChatView {
    /// Show a notification to the user
    pub fn show_notification(&mut self, message: impl Into<String>, notification_type: NotificationType, cx: &mut Context<Self>) {
        let msg = message.into();
        // Auto-detect quick action based on message content
        let quick_action = get_notification_action(&msg, &notification_type);

        let notification = Notification {
            message: msg,
            notification_type,
            created_at: chrono::Utc::now(),
            quick_action,
        };
        self.notifications.push(notification);
        // Auto-dismiss after 4 seconds (longer if has action)
        let duration = if quick_action.is_some() { 5 } else { 3 };
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(std::time::Duration::from_secs(duration)).await;
            let _ = this.update(cx, |view, cx| {
                // Remove oldest notification
                if !view.notifications.is_empty() {
                    view.notifications.remove(0);
                    cx.notify();
                }
            });
        }).detach();
        cx.notify();
    }

    /// Show a notification with a specific quick action
    pub fn show_notification_with_action(
        &mut self,
        message: impl Into<String>,
        notification_type: NotificationType,
        action: (&'static str, &'static str, &'static str),
        cx: &mut Context<Self>
    ) {
        let notification = Notification {
            message: message.into(),
            notification_type,
            created_at: chrono::Utc::now(),
            quick_action: Some(action),
        };
        self.notifications.push(notification);
        // Longer display time for actionable notifications
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(std::time::Duration::from_secs(6)).await;
            let _ = this.update(cx, |view, cx| {
                if !view.notifications.is_empty() {
                    view.notifications.remove(0);
                    cx.notify();
                }
            });
        }).detach();
        cx.notify();
    }

    /// Dismiss a specific notification by index
    pub fn dismiss_notification(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.notifications.len() {
            self.notifications.remove(index);
            cx.notify();
        }
    }

    /// Clear all notifications
    pub fn clear_notifications(&mut self, cx: &mut Context<Self>) {
        if !self.notifications.is_empty() {
            self.notifications.clear();
            cx.notify();
        }
    }

    /// Check if there are any active notifications
    pub fn has_notifications(&self) -> bool {
        !self.notifications.is_empty()
    }
}
