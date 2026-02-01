//! Types and events for Team panel

use crate::cloud::team::{ProjectPermission, TeamRole};

/// Events emitted by the team panel
pub enum TeamPanelEvent {
    /// Create new team
    CreateTeam {
        name: String,
        description: Option<String>,
    },
    /// Delete team
    DeleteTeam(String),
    /// Select team
    SelectTeam(String),
    /// Invite member
    InviteMember {
        team_id: String,
        email: String,
        role: TeamRole,
        message: Option<String>,
    },
    /// Remove member
    RemoveMember { team_id: String, member_id: String },
    /// Change member role
    ChangeMemberRole {
        team_id: String,
        member_id: String,
        new_role: TeamRole,
    },
    /// Share project with team
    ShareProject {
        team_id: String,
        project_id: String,
        permission: ProjectPermission,
    },
    /// Unshare project
    UnshareProject { team_id: String, project_id: String },
    /// Accept invitation
    AcceptInvitation(String),
    /// Decline invitation
    DeclineInvitation(String),
    /// Open project
    OpenProject(String),
    /// Panel closed
    Closed,
}

/// View modes for team panel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeamViewMode {
    /// Team list
    TeamList,
    /// Team details
    TeamDetails,
    /// Members view
    Members,
    /// Projects view
    Projects,
    /// Invitations view
    Invitations,
}
