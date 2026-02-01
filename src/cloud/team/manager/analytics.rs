//! Analytics operations

use chrono::Utc;
use std::collections::HashMap;

use super::super::analytics::{AnalyticsPeriod, UsageAnalytics};
use super::super::types::TeamError;
use super::types::TeamManager;

impl TeamManager {
    /// Get usage analytics for team
    pub async fn get_analytics(
        &self,
        team_id: &str,
        period: AnalyticsPeriod,
    ) -> Result<UsageAnalytics, TeamError> {
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

        if !role.can_view_analytics() {
            return Err(TeamError::Unauthorized(
                "Cannot view analytics".to_string(),
            ));
        }

        let now = Utc::now();
        let period_start = now - chrono::Duration::days(period.days());

        // In production, this would aggregate from actual usage data
        Ok(UsageAnalytics {
            team_id: team_id.to_string(),
            period,
            period_start,
            period_end: now,
            total_conversations: 0,
            total_messages: 0,
            total_tokens: 0,
            active_users: team.active_member_count() as u64,
            usage_by_user: HashMap::new(),
            usage_by_project: HashMap::new(),
            daily_breakdown: Vec::new(),
        })
    }
}
