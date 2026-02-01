//! Chat view events

/// Events emitted by ChatView
pub enum ChatViewEvent {
    /// User submitted a message
    Submit(String),
    /// User requested to stop the current response
    StopRequested,
    /// Export conversation requested
    ExportRequested,
    /// Theme toggle requested
    ThemeToggleRequested,
    /// Refresh git status requested
    RefreshGitStatus,
    /// Cancel a specific task
    CancelTask(Option<String>),
    /// Open a file in editor
    OpenFile(String),
    /// File attached to context
    FileAttached(String),
    /// Permission response from user
    PermissionResponse {
        /// The request ID being responded to
        request_id: String,
        /// Whether permission was granted
        granted: bool,
    },
}

/// Notification type for user feedback
#[derive(Debug, Clone)]
pub enum NotificationType {
    Success,
    Info,
    Warning,
    Error,
}

/// A notification to show to the user
#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub notification_type: NotificationType,
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Optional quick action (icon, label, command)
    pub quick_action: Option<(&'static str, &'static str, &'static str)>,
}

impl Notification {
    /// Create a notification with a quick action
    pub fn with_action(
        message: impl Into<String>,
        notification_type: NotificationType,
        action: (&'static str, &'static str, &'static str),
    ) -> Self {
        Self {
            message: message.into(),
            notification_type,
            created_at: chrono::Utc::now(),
            quick_action: Some(action),
        }
    }
}

/// Get suggested action for a notification based on its content
pub fn get_notification_action(message: &str, notification_type: &NotificationType) -> Option<(&'static str, &'static str, &'static str)> {
    let msg_lower = message.to_lowercase();

    match notification_type {
        NotificationType::Error => {
            if msg_lower.contains("context") || msg_lower.contains("full") {
                Some(("🗜️", "Compact", "/compact"))
            } else if msg_lower.contains("auth") || msg_lower.contains("login") {
                Some(("🔑", "Login", "/login"))
            } else {
                Some(("🐛", "Debug", "/debug"))
            }
        }
        NotificationType::Warning => {
            if msg_lower.contains("context") {
                Some(("🗜️", "Compact", "/compact"))
            } else if msg_lower.contains("rate") || msg_lower.contains("limit") {
                Some(("📊", "Usage", "/usage"))
            } else {
                None
            }
        }
        NotificationType::Success => {
            if msg_lower.contains("commit") {
                Some(("🔀", "Create PR", "/create-pr"))
            } else if msg_lower.contains("saved") || msg_lower.contains("memory") {
                Some(("📝", "View Memory", "/memory"))
            } else {
                None
            }
        }
        NotificationType::Info => None,
    }
}
