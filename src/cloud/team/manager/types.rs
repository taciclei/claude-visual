//! Team manager types and core structure

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::super::activity::ActivityEntry;
use super::super::invitations::TeamInvitation;
use super::super::projects::SharedProject;
use super::super::types::Team;

/// Team manager for team operations
pub struct TeamManager {
    /// Current user ID
    pub(crate) user_id: Arc<RwLock<Option<String>>>,
    /// Teams cache
    pub(crate) teams: Arc<RwLock<HashMap<String, Team>>>,
    /// Shared projects cache
    pub(crate) projects: Arc<RwLock<HashMap<String, SharedProject>>>,
    /// Activity feed cache
    pub(crate) activities: Arc<RwLock<Vec<ActivityEntry>>>,
    /// Pending invitations
    pub(crate) invitations: Arc<RwLock<Vec<TeamInvitation>>>,
    /// API base URL
    pub(crate) api_url: String,
}
