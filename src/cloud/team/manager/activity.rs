//! Activity feed operations

use chrono::Utc;

use super::super::activity::{ActivityEntry, ActivityTarget, ActivityType};
use super::super::types::TeamError;
use super::types::TeamManager;

impl TeamManager {
    /// Log activity
    pub(crate) async fn log_activity(
        &self,
        team_id: &str,
        user_id: &str,
        activity_type: ActivityType,
        target_type: ActivityTarget,
        target_id: &str,
        target_name: Option<&str>,
    ) {
        let entry = ActivityEntry {
            id: uuid::Uuid::new_v4().to_string(),
            team_id: team_id.to_string(),
            user_id: user_id.to_string(),
            user_name: None, // Would be fetched from user profile
            user_avatar: None,
            activity_type,
            target_type,
            target_id: target_id.to_string(),
            target_name: target_name.map(String::from),
            details: None,
            timestamp: Utc::now(),
        };

        self.activities.write().await.push(entry);
    }

    /// Get activity feed for team
    pub async fn get_activity_feed(
        &self,
        team_id: &str,
        limit: usize,
    ) -> Result<Vec<ActivityEntry>, TeamError> {
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

        let activities = self.activities.read().await;
        let mut team_activities: Vec<_> = activities
            .iter()
            .filter(|a| a.team_id == team_id)
            .cloned()
            .collect();

        team_activities.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        team_activities.truncate(limit);

        Ok(team_activities)
    }
}
