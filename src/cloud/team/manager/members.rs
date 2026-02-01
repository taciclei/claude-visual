//! Member management and invitation operations

use chrono::Utc;

use super::super::activity::{ActivityTarget, ActivityType};
use super::super::invitations::{InvitationStatus, TeamInvitation};
use super::super::types::{Team, TeamError, TeamMember, TeamRole};
use super::types::TeamManager;

impl TeamManager {
    /// Invite member to team
    pub async fn invite_member(
        &self,
        team_id: &str,
        email: &str,
        role: TeamRole,
        message: Option<String>,
    ) -> Result<TeamInvitation, TeamError> {
        let user_id = self
            .user_id
            .read()
            .await
            .clone()
            .ok_or_else(|| TeamError::Unauthorized("Not logged in".to_string()))?;

        let team = self
            .teams
            .read()
            .await
            .get(team_id)
            .cloned()
            .ok_or_else(|| TeamError::NotFound(team_id.to_string()))?;

        let user_role = team
            .get_role(&user_id)
            .ok_or_else(|| TeamError::Unauthorized("Not a team member".to_string()))?;

        if !user_role.can_manage_members() && !team.settings.members_can_invite {
            return Err(TeamError::Unauthorized("Cannot invite members".to_string()));
        }

        let now = Utc::now();
        let invitation = TeamInvitation {
            id: uuid::Uuid::new_v4().to_string(),
            team_id: team_id.to_string(),
            team_name: team.name.clone(),
            inviter_id: user_id.clone(),
            inviter_name: team.get_member(&user_id).and_then(|m| m.name.clone()),
            invitee_email: email.to_string(),
            role,
            message,
            created_at: now,
            expires_at: now + chrono::Duration::days(7),
            status: InvitationStatus::Pending,
        };

        self.invitations.write().await.push(invitation.clone());

        self.log_activity(
            team_id,
            &user_id,
            ActivityType::Invited,
            ActivityTarget::Member,
            email,
            Some(email),
        )
        .await;

        Ok(invitation)
    }

    /// Accept invitation
    pub async fn accept_invitation(&self, invitation_id: &str) -> Result<Team, TeamError> {
        let user_id = self
            .user_id
            .read()
            .await
            .clone()
            .ok_or_else(|| TeamError::Unauthorized("Not logged in".to_string()))?;

        let mut invitations = self.invitations.write().await;
        let invitation = invitations
            .iter_mut()
            .find(|i| i.id == invitation_id)
            .ok_or_else(|| TeamError::NotFound(invitation_id.to_string()))?;

        if invitation.status != InvitationStatus::Pending {
            return Err(TeamError::InvalidOperation(format!(
                "Invitation is {:?}",
                invitation.status
            )));
        }

        if invitation.expires_at < Utc::now() {
            invitation.status = InvitationStatus::Expired;
            return Err(TeamError::InvalidOperation(
                "Invitation expired".to_string(),
            ));
        }

        invitation.status = InvitationStatus::Accepted;
        let team_id = invitation.team_id.clone();
        let role = invitation.role;
        drop(invitations);

        // Add member to team
        let mut teams = self.teams.write().await;
        let team = teams
            .get_mut(&team_id)
            .ok_or_else(|| TeamError::NotFound(team_id.clone()))?;

        let member = TeamMember {
            user_id: user_id.clone(),
            email: String::new(), // Would be fetched from user profile
            name: None,
            avatar_url: None,
            role,
            joined_at: Utc::now(),
            last_active: Some(Utc::now()),
            pending_invite: false,
        };

        team.members.push(member);
        team.updated_at = Utc::now();
        let team_clone = team.clone();
        drop(teams);

        self.log_activity(
            &team_id,
            &user_id,
            ActivityType::Joined,
            ActivityTarget::Team,
            &team_id,
            Some(&team_clone.name),
        )
        .await;

        Ok(team_clone)
    }

    /// Remove member from team
    pub async fn remove_member(&self, team_id: &str, member_id: &str) -> Result<(), TeamError> {
        let user_id = self
            .user_id
            .read()
            .await
            .clone()
            .ok_or_else(|| TeamError::Unauthorized("Not logged in".to_string()))?;

        let mut teams = self.teams.write().await;
        let team = teams
            .get_mut(team_id)
            .ok_or_else(|| TeamError::NotFound(team_id.to_string()))?;

        let user_role = team
            .get_role(&user_id)
            .ok_or_else(|| TeamError::Unauthorized("Not a team member".to_string()))?;

        // Can only remove self or if admin
        if user_id != member_id && !user_role.can_manage_members() {
            return Err(TeamError::Unauthorized(
                "Cannot remove other members".to_string(),
            ));
        }

        // Cannot remove owner
        if let Some(member) = team.get_member(member_id) {
            if member.role == TeamRole::Owner {
                return Err(TeamError::InvalidOperation(
                    "Cannot remove team owner".to_string(),
                ));
            }
        }

        team.members.retain(|m| m.user_id != member_id);
        team.updated_at = Utc::now();
        let team_name = team.name.clone();
        drop(teams);

        self.log_activity(
            team_id,
            &user_id,
            ActivityType::Left,
            ActivityTarget::Member,
            member_id,
            Some(&team_name),
        )
        .await;

        Ok(())
    }

    /// Change member role
    pub async fn change_member_role(
        &self,
        team_id: &str,
        member_id: &str,
        new_role: TeamRole,
    ) -> Result<(), TeamError> {
        let user_id = self
            .user_id
            .read()
            .await
            .clone()
            .ok_or_else(|| TeamError::Unauthorized("Not logged in".to_string()))?;

        let mut teams = self.teams.write().await;
        let team = teams
            .get_mut(team_id)
            .ok_or_else(|| TeamError::NotFound(team_id.to_string()))?;

        let user_role = team
            .get_role(&user_id)
            .ok_or_else(|| TeamError::Unauthorized("Not a team member".to_string()))?;

        if !user_role.can_manage_members() {
            return Err(TeamError::Unauthorized(
                "Cannot change member roles".to_string(),
            ));
        }

        // Cannot change owner role
        if let Some(member) = team.members.iter_mut().find(|m| m.user_id == member_id) {
            if member.role == TeamRole::Owner {
                return Err(TeamError::InvalidOperation(
                    "Cannot change owner role".to_string(),
                ));
            }
            member.role = new_role;
        }

        team.updated_at = Utc::now();
        drop(teams);

        self.log_activity(
            team_id,
            &user_id,
            ActivityType::RoleChanged,
            ActivityTarget::Member,
            member_id,
            Some(new_role.display_name()),
        )
        .await;

        Ok(())
    }

    /// Get pending invitations for current user
    pub async fn get_pending_invitations(&self) -> Result<Vec<TeamInvitation>, TeamError> {
        let user_id = self
            .user_id
            .read()
            .await
            .clone()
            .ok_or_else(|| TeamError::Unauthorized("Not logged in".to_string()))?;

        let invitations = self.invitations.read().await;
        Ok(invitations
            .iter()
            .filter(|i| i.status == InvitationStatus::Pending)
            .filter(|i| {
                // Would check if user email matches invitee_email
                // For now, just return all pending
                true
            })
            .cloned()
            .collect())
    }
}
