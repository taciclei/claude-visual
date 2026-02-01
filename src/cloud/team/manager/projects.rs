//! Project sharing operations

use chrono::Utc;

use super::super::activity::{ActivityTarget, ActivityType};
use super::super::projects::{ProjectPermission, SharedProject};
use super::super::types::TeamError;
use super::types::TeamManager;

impl TeamManager {
    /// Share project with team
    pub async fn share_project(
        &self,
        team_id: &str,
        project_id: &str,
        project_name: &str,
        permission: ProjectPermission,
    ) -> Result<SharedProject, TeamError> {
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

        if !role.can_manage_projects() && !team.settings.members_can_create_projects {
            return Err(TeamError::Unauthorized(
                "Cannot share projects".to_string(),
            ));
        }

        let now = Utc::now();
        let shared = SharedProject {
            id: project_id.to_string(),
            name: project_name.to_string(),
            description: None,
            team_id: team_id.to_string(),
            owner_id: user_id.clone(),
            shared_with: Vec::new(),
            team_permission: permission,
            created_at: now,
            updated_at: now,
            last_activity: now,
        };

        self.projects
            .write()
            .await
            .insert(project_id.to_string(), shared.clone());

        // Add to team's project list
        self.teams
            .write()
            .await
            .get_mut(team_id)
            .map(|t| t.project_ids.push(project_id.to_string()));

        self.log_activity(
            team_id,
            &user_id,
            ActivityType::Shared,
            ActivityTarget::Project,
            project_id,
            Some(project_name),
        )
        .await;

        Ok(shared)
    }

    /// List team projects
    pub async fn list_team_projects(&self, team_id: &str) -> Result<Vec<SharedProject>, TeamError> {
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

        if !team.is_member(&user_id) {
            return Err(TeamError::Unauthorized("Not a team member".to_string()));
        }

        let projects = self.projects.read().await;
        Ok(projects
            .values()
            .filter(|p| p.team_id == team_id)
            .cloned()
            .collect())
    }

    /// Unshare project from team
    pub async fn unshare_project(&self, team_id: &str, project_id: &str) -> Result<(), TeamError> {
        let user_id = self
            .user_id
            .read()
            .await
            .clone()
            .ok_or_else(|| TeamError::Unauthorized("Not logged in".to_string()))?;

        let project = self
            .projects
            .read()
            .await
            .get(project_id)
            .cloned()
            .ok_or_else(|| TeamError::NotFound(project_id.to_string()))?;

        // Only owner or admin can unshare
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

        if project.owner_id != user_id && !role.can_manage_projects() {
            return Err(TeamError::Unauthorized(
                "Cannot unshare this project".to_string(),
            ));
        }

        self.projects.write().await.remove(project_id);

        // Remove from team's project list
        self.teams
            .write()
            .await
            .get_mut(team_id)
            .map(|t| t.project_ids.retain(|id| id != project_id));

        self.log_activity(
            team_id,
            &user_id,
            ActivityType::Deleted,
            ActivityTarget::Project,
            project_id,
            Some(&project.name),
        )
        .await;

        Ok(())
    }
}
