//! Core types for the extension API

use std::path::PathBuf;

/// Extension API context passed to plugins
#[derive(Debug, Clone)]
pub struct ExtensionContext {
    /// Extension ID
    pub(crate) extension_id: String,
    /// Current working directory
    pub(crate) working_directory: Option<PathBuf>,
    /// Current project path
    pub(crate) project_path: Option<PathBuf>,
}

impl ExtensionContext {
    /// Create a new extension context
    pub fn new(extension_id: String) -> Self {
        Self {
            extension_id,
            working_directory: None,
            project_path: None,
        }
    }

    /// Set the working directory
    pub fn with_working_directory(mut self, path: PathBuf) -> Self {
        self.working_directory = Some(path);
        self
    }

    /// Set the project path
    pub fn with_project_path(mut self, path: PathBuf) -> Self {
        self.project_path = Some(path);
        self
    }
}

/// Result type returned by extension API calls
#[derive(Debug, Clone)]
pub enum ApiResult {
    /// Success with optional data
    Success(Option<String>),
    /// Error with message
    Error(String),
}

impl ApiResult {
    /// Create a success result
    pub fn ok() -> Self {
        Self::Success(None)
    }

    /// Create a success result with data
    pub fn data(data: String) -> Self {
        Self::Success(Some(data))
    }

    /// Create an error result
    pub fn error(message: impl Into<String>) -> Self {
        Self::Error(message.into())
    }

    /// Check if the result is successful
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Success(_))
    }

    /// Get the data if successful
    pub fn get_data(&self) -> Option<&str> {
        match self {
            Self::Success(Some(data)) => Some(data),
            _ => None,
        }
    }

    /// Get the error message if failed
    pub fn get_error(&self) -> Option<&str> {
        match self {
            Self::Error(msg) => Some(msg),
            _ => None,
        }
    }
}

/// Notification to display to the user
#[derive(Debug, Clone)]
pub struct Notification {
    /// Notification ID
    pub(crate) id: String,
    /// Title
    pub(crate) title: String,
    /// Body text
    pub(crate) body: Option<String>,
    /// Severity level
    pub(crate) level: NotificationLevel,
    /// Extension that created this notification
    pub(crate) extension_id: String,
}

/// Notification severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

/// Status bar item
#[derive(Debug, Clone)]
pub struct StatusItem {
    /// Item ID
    pub(crate) id: String,
    /// Display text
    pub(crate) text: String,
    /// Tooltip text
    pub(crate) tooltip: Option<String>,
    /// Icon name
    pub(crate) icon: Option<String>,
    /// Extension that created this item
    pub(crate) extension_id: String,
}

/// Event subscription
#[derive(Debug, Clone)]
pub struct EventSubscription {
    /// Subscription ID
    pub(crate) id: String,
    /// Extension that created this subscription
    pub(crate) extension_id: String,
    /// Callback function index in WASM
    pub(crate) callback_index: u32,
}
