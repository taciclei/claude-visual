//! Notification types and enums

use gpui::*;

/// Notification type/priority
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum NotificationType {
    /// Informational (default)
    #[default]
    Info,
    /// Success
    Success,
    /// Warning
    Warning,
    /// Error
    Error,
    /// System/critical
    System,
}

impl NotificationType {
    pub(crate) fn icon(&self) -> &str {
        match self {
            NotificationType::Info => "ℹ️",
            NotificationType::Success => "✓",
            NotificationType::Warning => "⚠️",
            NotificationType::Error => "✕",
            NotificationType::System => "⚙️",
        }
    }

    pub(crate) fn color(&self) -> Hsla {
        match self {
            NotificationType::Info => hsla(0.6, 0.8, 0.6, 1.0),
            NotificationType::Success => hsla(0.38, 0.7, 0.45, 1.0),
            NotificationType::Warning => hsla(0.12, 0.9, 0.5, 1.0),
            NotificationType::Error => hsla(0.0, 0.7, 0.5, 1.0),
            NotificationType::System => hsla(0.75, 0.6, 0.55, 1.0),
        }
    }
}

/// Events emitted by notifications
#[derive(Debug, Clone)]
pub enum NotificationEvent {
    /// Notification clicked
    Clicked(String),
    /// Notification dismissed
    Dismissed(String),
    /// Action clicked
    ActionClicked { notification_id: String, action_id: String },
    /// Mark as read
    MarkedAsRead(String),
    /// Clear all
    ClearedAll,
}

/// Notification action button
#[derive(Clone)]
pub struct NotificationAction {
    pub id: String,
    pub label: String,
    pub primary: bool,
}
