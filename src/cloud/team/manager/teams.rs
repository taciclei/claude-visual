//! Team CRUD operations

use chrono::Utc;

use super::super::activity::{ActivityTarget, ActivityType};
use super::super::types::{Team, TeamError};
use super::types::TeamManager;

impl TeamManager {
    /// Create a new team
    pub async fn create_team(
        &self,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Result<Team, TeamError> {
        let user_id = self
            .user_id
            .read()
            .await
            .clone()
            .ok_or_else(|| TeamError::Unauthorized("Not logged in".to_string()))?;

        let mut team = Team::new(name, user_id.clone(), String::new());
        team.description = description;

        // In production, this would call the API
        // For now, store locally
        self.teams
            .write()
            .await
            .insert(team.id.clone(), team.clone());

        // Log activity
        self.log_activity(
            &team.id,
            &user_id,
            ActivityType::Created,
            ActivityTarget::Team,
            &team.id,
            Some(&team.name),
        )
        .await;

        Ok(team)
    }

    /// Get team by ID
    pub async fn get_team(&self, team_id: &str) -> Result<Option<Team>, TeamError> {
        Ok(self.teams.read().await.get(team_id).cloned())
    }

    /// List user's teams
    pub async fn list_teams(&self) -> Result<Vec<Team>, TeamError> {
        let user_id = self
            .user_id
            .read()
            .await
            .clone()
            .ok_or_else(|| TeamError::Unauthorized("Not logged in".to_string()))?;

        let teams = self.teams.read().await;
        Ok(teams
            .values()
            .filter(|t| t.is_member(&user_id))
            .cloned()
            .collect())
    }

    /// Update team
    pub async fn update_team(&self, team: &Team) -> Result<(), TeamError> {
        let user_id = self
            .user_id
            .read()
            .await
            .clone()
            .ok_or_else(|| TeamError::Unauthorized("Not logged in".to_string()))?;

        let role = team
            .get_role(&user_id)
            .ok_or_else(|| TeamError::Unauthorized("Not a team member".to_string()))?;

        if !role.can_manage_members() {
            return Err(TeamError::Unauthorized(
                "Insufficient permissions".to_string(),
            ));
        }

        let mut updated = team.clone();
        updated.updated_at = Utc::now();
        self.teams.write().await.insert(team.id.clone(), updated);

        self.log_activity(
            &team.id,
            &user_id,
            ActivityType::Updated,
            ActivityTarget::Team,
            &team.id,
            Some(&team.name),
        )
        .await;

        Ok(())
    }

    /// Delete team
    pub async fn delete_team(&self, team_id: &str) -> Result<(), TeamError> {
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

        let role = team
            .get_role(&user_id)
            .ok_or_else(|| TeamError::Unauthorized("Not a team member".to_string()))?;

        if !role.can_delete_team() {
            return Err(TeamError::Unauthorized(
                "Only owner can delete team".to_string(),
            ));
        }

        self.teams.write().await.remove(team_id);

        // Remove associated projects
        let mut projects = self.projects.write().await;
        projects.retain(|_, p| p.team_id != team_id);

        Ok(())
    }
}
