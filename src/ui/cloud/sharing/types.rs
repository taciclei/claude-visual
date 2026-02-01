//! Sharing types

use serde::{Deserialize, Serialize};

/// Share permission levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharePermission {
    /// View only
    View,
    /// View and comment
    Comment,
    /// Full edit access
    Edit,
}

impl SharePermission {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            SharePermission::View => "View only",
            SharePermission::Comment => "Can comment",
            SharePermission::Edit => "Can edit",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            SharePermission::View => "Recipients can view the conversation",
            SharePermission::Comment => "Recipients can view and add comments",
            SharePermission::Edit => "Recipients can view, comment, and edit",
        }
    }
}

/// Share link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareLink {
    /// Link ID
    pub id: String,
    /// Full URL
    pub url: String,
    /// Permission level
    pub permission: SharePermission,
    /// Password protected
    pub password_protected: bool,
    /// Expiry date
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Created at
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Access count
    pub access_count: u32,
}

/// Events emitted by the share dialog
pub enum ShareDialogEvent {
    /// Generate new share link
    GenerateLink {
        conversation_id: String,
        permission: SharePermission,
        password: Option<String>,
        expires_in: Option<chrono::Duration>,
    },
    /// Revoke a share link
    RevokeLink(String),
    /// Copy link to clipboard
    CopyLink(String),
    /// Dialog closed
    Closed,
}

/// Expiry option
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpiryOption {
    Never,
    OneHour,
    OneDay,
    OneWeek,
    OneMonth,
    Custom,
}

impl ExpiryOption {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ExpiryOption::Never => "Never",
            ExpiryOption::OneHour => "1 hour",
            ExpiryOption::OneDay => "1 day",
            ExpiryOption::OneWeek => "1 week",
            ExpiryOption::OneMonth => "1 month",
            ExpiryOption::Custom => "Custom...",
        }
    }

    /// Convert to duration
    pub fn to_duration(&self) -> Option<chrono::Duration> {
        match self {
            ExpiryOption::Never => None,
            ExpiryOption::OneHour => Some(chrono::Duration::hours(1)),
            ExpiryOption::OneDay => Some(chrono::Duration::days(1)),
            ExpiryOption::OneWeek => Some(chrono::Duration::weeks(1)),
            ExpiryOption::OneMonth => Some(chrono::Duration::days(30)),
            ExpiryOption::Custom => None, // Handled separately
        }
    }
}
