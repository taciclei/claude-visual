//! Activity tracking types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Activity feed entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEntry {
    /// Unique entry ID
    pub id: String,
    /// Team ID
    pub team_id: String,
    /// User who performed action
    pub user_id: String,
    /// User display name
    pub user_name: Option<String>,
    /// User avatar
    pub user_avatar: Option<String>,
    /// Activity type
    pub activity_type: ActivityType,
    /// Target entity type
    pub target_type: ActivityTarget,
    /// Target entity ID
    pub target_id: String,
    /// Target entity name
    pub target_name: Option<String>,
    /// Additional details
    pub details: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Activity type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityType {
    /// Created something
    Created,
    /// Updated something
    Updated,
    /// Deleted something
    Deleted,
    /// Shared something
    Shared,
    /// Joined team
    Joined,
    /// Left team
    Left,
    /// Invited member
    Invited,
    /// Changed role
    RoleChanged,
    /// Commented
    Commented,
    /// Started conversation
    ConversationStarted,
    /// Completed task
    TaskCompleted,
}

impl ActivityType {
    pub fn verb(&self) -> &'static str {
        match self {
            ActivityType::Created => "created",
            ActivityType::Updated => "updated",
            ActivityType::Deleted => "deleted",
            ActivityType::Shared => "shared",
            ActivityType::Joined => "joined",
            ActivityType::Left => "left",
            ActivityType::Invited => "invited",
            ActivityType::RoleChanged => "changed role for",
            ActivityType::Commented => "commented on",
            ActivityType::ConversationStarted => "started a conversation in",
            ActivityType::TaskCompleted => "completed a task in",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ActivityType::Created => "+",
            ActivityType::Updated => "~",
            ActivityType::Deleted => "-",
            ActivityType::Shared => "→",
            ActivityType::Joined => "↓",
            ActivityType::Left => "↑",
            ActivityType::Invited => "✉",
            ActivityType::RoleChanged => "⚙",
            ActivityType::Commented => "💬",
            ActivityType::ConversationStarted => "🗨",
            ActivityType::TaskCompleted => "✓",
        }
    }
}

/// Activity target type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityTarget {
    /// Team itself
    Team,
    /// Project
    Project,
    /// Conversation
    Conversation,
    /// Member
    Member,
    /// File
    File,
    /// Comment
    Comment,
}

impl ActivityTarget {
    pub fn display_name(&self) -> &'static str {
        match self {
            ActivityTarget::Team => "team",
            ActivityTarget::Project => "project",
            ActivityTarget::Conversation => "conversation",
            ActivityTarget::Member => "member",
            ActivityTarget::File => "file",
            ActivityTarget::Comment => "comment",
        }
    }
}
