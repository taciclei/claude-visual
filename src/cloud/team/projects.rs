//! Project-related types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Shared project within a team
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedProject {
    /// Project ID
    pub id: String,
    /// Project name
    pub name: String,
    /// Project description
    pub description: Option<String>,
    /// Team ID
    pub team_id: String,
    /// Owner user ID
    pub owner_id: String,
    /// Shared with specific members (empty = all team)
    pub shared_with: Vec<String>,
    /// Permission level for team members
    pub team_permission: ProjectPermission,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Last activity
    pub last_activity: DateTime<Utc>,
}

/// Project permission level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectPermission {
    /// Read-only access
    View,
    /// Can comment
    Comment,
    /// Full edit access
    Edit,
    /// Full control including delete
    Admin,
}

impl ProjectPermission {
    pub fn display_name(&self) -> &'static str {
        match self {
            ProjectPermission::View => "View",
            ProjectPermission::Comment => "Comment",
            ProjectPermission::Edit => "Edit",
            ProjectPermission::Admin => "Admin",
        }
    }
}
