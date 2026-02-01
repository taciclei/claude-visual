//! Core team types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Team error types
#[derive(Debug, thiserror::Error)]
pub enum TeamError {
    /// Not authorized for this action
    #[error("Not authorized: {0}")]
    Unauthorized(String),
    /// Team not found
    #[error("Team not found: {0}")]
    NotFound(String),
    /// Invalid operation
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    /// Network error
    #[error("Network error: {0}")]
    Network(String),
    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),
}

/// Team member role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamRole {
    /// Team owner with full permissions
    Owner,
    /// Admin with management permissions
    Admin,
    /// Regular member
    Member,
    /// Read-only viewer
    Viewer,
}

impl TeamRole {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            TeamRole::Owner => "Owner",
            TeamRole::Admin => "Admin",
            TeamRole::Member => "Member",
            TeamRole::Viewer => "Viewer",
        }
    }

    /// Check if role can manage members
    pub fn can_manage_members(&self) -> bool {
        matches!(self, TeamRole::Owner | TeamRole::Admin)
    }

    /// Check if role can manage projects
    pub fn can_manage_projects(&self) -> bool {
        matches!(self, TeamRole::Owner | TeamRole::Admin | TeamRole::Member)
    }

    /// Check if role can delete team
    pub fn can_delete_team(&self) -> bool {
        matches!(self, TeamRole::Owner)
    }

    /// Check if role can view analytics
    pub fn can_view_analytics(&self) -> bool {
        matches!(self, TeamRole::Owner | TeamRole::Admin)
    }
}

/// Team member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    /// User ID
    pub user_id: String,
    /// User email
    pub email: String,
    /// Display name
    pub name: Option<String>,
    /// Avatar URL
    pub avatar_url: Option<String>,
    /// Role in team
    pub role: TeamRole,
    /// When joined
    pub joined_at: DateTime<Utc>,
    /// Last active
    pub last_active: Option<DateTime<Utc>>,
    /// Invitation pending
    pub pending_invite: bool,
}

/// Team workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    /// Unique team ID
    pub id: String,
    /// Team name
    pub name: String,
    /// Team description
    pub description: Option<String>,
    /// Team avatar URL
    pub avatar_url: Option<String>,
    /// Team members
    pub members: Vec<TeamMember>,
    /// Shared project IDs
    pub project_ids: Vec<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Team settings
    pub settings: TeamSettings,
}

impl Team {
    /// Create a new team with owner
    pub fn new(name: impl Into<String>, owner_id: String, owner_email: String) -> Self {
        let now = Utc::now();
        let owner = TeamMember {
            user_id: owner_id,
            email: owner_email,
            name: None,
            avatar_url: None,
            role: TeamRole::Owner,
            joined_at: now,
            last_active: Some(now),
            pending_invite: false,
        };

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            description: None,
            avatar_url: None,
            members: vec![owner],
            project_ids: Vec::new(),
            created_at: now,
            updated_at: now,
            settings: TeamSettings::default(),
        }
    }

    /// Get member by user ID
    pub fn get_member(&self, user_id: &str) -> Option<&TeamMember> {
        self.members.iter().find(|m| m.user_id == user_id)
    }

    /// Get member role
    pub fn get_role(&self, user_id: &str) -> Option<TeamRole> {
        self.get_member(user_id).map(|m| m.role)
    }

    /// Check if user is member
    pub fn is_member(&self, user_id: &str) -> bool {
        self.get_member(user_id).is_some()
    }

    /// Count active members (non-pending)
    pub fn active_member_count(&self) -> usize {
        self.members.iter().filter(|m| !m.pending_invite).count()
    }
}

/// Team settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamSettings {
    /// Allow members to invite others
    pub members_can_invite: bool,
    /// Allow members to create projects
    pub members_can_create_projects: bool,
    /// Default role for new members
    pub default_role: TeamRole,
    /// Require approval for new members
    pub require_approval: bool,
    /// Enable activity notifications
    pub activity_notifications: bool,
    /// Enable usage analytics
    pub analytics_enabled: bool,
}

impl Default for TeamSettings {
    fn default() -> Self {
        Self {
            members_can_invite: false,
            members_can_create_projects: true,
            default_role: TeamRole::Member,
            require_approval: true,
            activity_notifications: true,
            analytics_enabled: true,
        }
    }
}
