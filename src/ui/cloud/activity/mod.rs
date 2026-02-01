//! Activity Feed UI
//!
//! Panel for displaying team activity feed.

mod entry;
mod events;
mod filters;
mod render;

use std::sync::Arc;

use gpui::*;

use crate::app::state::AppState;
use crate::cloud::team::{ActivityEntry, ActivityType};

pub use events::ActivityPanelEvent;

/// Activity feed panel
pub struct ActivityPanel {
    pub(super) app_state: Arc<AppState>,
    /// Team ID
    team_id: Option<String>,
    /// Team name
    team_name: Option<String>,
    /// Activity entries
    pub(super) activities: Vec<ActivityEntry>,
    /// Is loading
    pub(super) is_loading: bool,
    /// Error message
    error_message: Option<String>,
    /// Filter by activity type
    pub(super) filter_type: Option<ActivityType>,
    /// Filter by user
    pub(super) filter_user: Option<String>,
    /// Focus handle
    focus_handle: FocusHandle,
}

impl ActivityPanel {
    /// Create a new activity panel
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            team_id: None,
            team_name: None,
            activities: Vec::new(),
            is_loading: false,
            error_message: None,
            filter_type: None,
            filter_user: None,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Set team
    pub fn set_team(&mut self, team_id: String, team_name: String, cx: &mut Context<Self>) {
        self.team_id = Some(team_id);
        self.team_name = Some(team_name);
        cx.notify();
    }

    /// Set activities
    pub fn set_activities(&mut self, activities: Vec<ActivityEntry>, cx: &mut Context<Self>) {
        self.activities = activities;
        self.is_loading = false;
        cx.notify();
    }

    /// Set loading state
    pub fn set_loading(&mut self, loading: bool, cx: &mut Context<Self>) {
        self.is_loading = loading;
        cx.notify();
    }

    /// Set error
    pub fn set_error(&mut self, error: Option<String>, cx: &mut Context<Self>) {
        self.error_message = error;
        self.is_loading = false;
        cx.notify();
    }
}
