//! TeamManager tests

#[cfg(test)]
mod tests {
    use super::super::types::TeamManager;
    use crate::cloud::team::activity::ActivityType;
    use crate::cloud::team::types::TeamRole;

    #[tokio::test]
    async fn test_team_creation() {
        let manager = TeamManager::new("https://api.example.com");
        manager.set_user(Some("user1".to_string())).await;

        let team = manager
            .create_team("Test Team", Some("A test team".to_string()))
            .await
            .unwrap();

        assert_eq!(team.name, "Test Team");
        assert_eq!(team.members.len(), 1);
        assert_eq!(team.members[0].role, TeamRole::Owner);
    }

    #[tokio::test]
    async fn test_member_roles() {
        assert!(TeamRole::Owner.can_manage_members());
        assert!(TeamRole::Admin.can_manage_members());
        assert!(!TeamRole::Member.can_manage_members());
        assert!(!TeamRole::Viewer.can_manage_members());

        assert!(TeamRole::Owner.can_delete_team());
        assert!(!TeamRole::Admin.can_delete_team());
    }

    #[tokio::test]
    async fn test_activity_logging() {
        let manager = TeamManager::new("https://api.example.com");
        manager.set_user(Some("user1".to_string())).await;

        let team = manager.create_team("Test Team", None).await.unwrap();

        let activities = manager.get_activity_feed(&team.id, 10).await.unwrap();
        assert!(!activities.is_empty());
        assert_eq!(activities[0].activity_type, ActivityType::Created);
    }
}
