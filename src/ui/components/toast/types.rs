//! Toast types and data structures

use std::time::Duration;

/// Toast notification level/type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastLevel {
    /// Informational message
    Info,
    /// Success message
    Success,
    /// Warning message
    Warning,
    /// Error message
    Error,
}

impl ToastLevel {
    /// Get the icon for this level
    pub fn icon(&self) -> &'static str {
        match self {
            ToastLevel::Info => "i",
            ToastLevel::Success => "v",
            ToastLevel::Warning => "!",
            ToastLevel::Error => "x",
        }
    }
}

/// A single toast notification
#[derive(Debug, Clone)]
pub struct Toast {
    /// Unique identifier
    pub(crate) id: usize,
    /// Notification level
    pub(crate) level: ToastLevel,
    /// Message to display
    pub(crate) message: String,
    /// Optional action button text
    pub(crate) action: Option<String>,
    /// Whether the toast is dismissible
    pub(crate) dismissible: bool,
    /// Duration before auto-dismiss (None = persistent)
    pub(crate) duration: Option<Duration>,
}

impl Toast {
    /// Create a new info toast
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            id: 0,
            level: ToastLevel::Info,
            message: message.into(),
            action: None,
            dismissible: true,
            duration: Some(Duration::from_secs(3)),
        }
    }

    /// Create a new success toast
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            id: 0,
            level: ToastLevel::Success,
            message: message.into(),
            action: None,
            dismissible: true,
            duration: Some(Duration::from_secs(3)),
        }
    }

    /// Create a new warning toast
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            id: 0,
            level: ToastLevel::Warning,
            message: message.into(),
            action: None,
            dismissible: true,
            duration: Some(Duration::from_secs(5)),
        }
    }

    /// Create a new error toast
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            id: 0,
            level: ToastLevel::Error,
            message: message.into(),
            action: None,
            dismissible: true,
            duration: Some(Duration::from_secs(5)),
        }
    }

    /// Set the action button text
    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Make the toast persistent (no auto-dismiss)
    pub fn persistent(mut self) -> Self {
        self.duration = None;
        self
    }

    /// Make the toast non-dismissible
    pub fn non_dismissible(mut self) -> Self {
        self.dismissible = false;
        self
    }

    /// Set custom duration
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }
}

/// Events emitted by ToastContainer
pub enum ToastEvent {
    /// Toast was dismissed by user
    Dismissed(usize),
    /// Toast action was clicked
    ActionClicked(usize),
}
