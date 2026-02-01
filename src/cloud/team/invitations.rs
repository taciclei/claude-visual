//! Team invitation types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::types::TeamRole;

/// Team invitation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamInvitation {
    /// Invitation ID
    pub id: String,
    /// Team ID
    pub team_id: String,
    /// Team name
    pub team_name: String,
    /// Inviter user ID
    pub inviter_id: String,
    /// Inviter name
    pub inviter_name: Option<String>,
    /// Invitee email
    pub invitee_email: String,
    /// Assigned role
    pub role: TeamRole,
    /// Invitation message
    pub message: Option<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Expires timestamp
    pub expires_at: DateTime<Utc>,
    /// Invitation status
    pub status: InvitationStatus,
}

/// Invitation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvitationStatus {
    /// Pending response
    Pending,
    /// Accepted
    Accepted,
    /// Declined
    Declined,
    /// Expired
    Expired,
    /// Revoked by admin
    Revoked,
}
