//! Team Workspace UI
//!
//! Panel for managing team workspaces, members, and shared projects.

use std::sync::Arc;

use gpui::*;

use crate::app::state::AppState;
use crate::cloud::team::{Team, TeamInvitation, TeamRole};

mod render;
mod types;
mod views;

pub use types::{TeamPanelEvent, TeamViewMode};

impl EventEmitter<TeamPanelEvent> for TeamPanel {}

/// Team panel view
pub struct TeamPanel {
    pub(crate) app_state: Arc<AppState>,
    /// Available teams
    pub(crate) teams: Vec<Team>,
    /// Selected team index
    pub(crate) selected_team: Option<usize>,
    /// Pending invitations
    pub(crate) invitations: Vec<TeamInvitation>,
    /// Current view mode
    pub(crate) view_mode: TeamViewMode,
    /// Create team dialog open
    pub(crate) create_dialog_open: bool,
    /// Invite member dialog open
    pub(crate) invite_dialog_open: bool,
    /// New team name input
    pub(crate) new_team_name: String,
    /// New team description input
    pub(crate) new_team_description: String,
    /// Invite email input
    pub(crate) invite_email: String,
    /// Invite role selection
    pub(crate) invite_role: TeamRole,
    /// Invite message input
    pub(crate) invite_message: String,
    /// Error message
    pub(crate) error_message: Option<String>,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}

impl TeamPanel {
    /// Create a new team panel
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            teams: Vec::new(),
            selected_team: None,
            invitations: Vec::new(),
            view_mode: TeamViewMode::TeamList,
            create_dialog_open: false,
            invite_dialog_open: false,
            new_team_name: String::new(),
            new_team_description: String::new(),
            invite_email: String::new(),
            invite_role: TeamRole::Member,
            invite_message: String::new(),
            error_message: None,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Set teams
    pub fn set_teams(&mut self, teams: Vec<Team>, cx: &mut Context<Self>) {
        self.teams = teams;
        cx.notify();
    }

    /// Set invitations
    pub fn set_invitations(&mut self, invitations: Vec<TeamInvitation>, cx: &mut Context<Self>) {
        self.invitations = invitations;
        cx.notify();
    }

    /// Get selected team
    pub fn selected_team(&self) -> Option<&Team> {
        self.selected_team.and_then(|i| self.teams.get(i))
    }

    /// Select team by index
    fn select_team(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.teams.len() {
            self.selected_team = Some(index);
            self.view_mode = TeamViewMode::TeamDetails;
            if let Some(team) = self.teams.get(index) {
                cx.emit(TeamPanelEvent::SelectTeam(team.id.clone()));
            }
            cx.notify();
        }
    }

    /// Open create team dialog
    pub fn open_create_dialog(&mut self, cx: &mut Context<Self>) {
        self.create_dialog_open = true;
        self.new_team_name.clear();
        self.new_team_description.clear();
        cx.notify();
    }

    /// Close create team dialog
    fn close_create_dialog(&mut self, cx: &mut Context<Self>) {
        self.create_dialog_open = false;
        cx.notify();
    }

    /// Create new team
    fn create_team(&mut self, cx: &mut Context<Self>) {
        if self.new_team_name.trim().is_empty() {
            self.error_message = Some("Team name is required".to_string());
            cx.notify();
            return;
        }

        cx.emit(TeamPanelEvent::CreateTeam {
            name: self.new_team_name.clone(),
            description: if self.new_team_description.is_empty() {
                None
            } else {
                Some(self.new_team_description.clone())
            },
        });

        self.close_create_dialog(cx);
    }

    /// Open invite dialog
    pub fn open_invite_dialog(&mut self, cx: &mut Context<Self>) {
        self.invite_dialog_open = true;
        self.invite_email.clear();
        self.invite_role = TeamRole::Member;
        self.invite_message.clear();
        cx.notify();
    }

    /// Close invite dialog
    fn close_invite_dialog(&mut self, cx: &mut Context<Self>) {
        self.invite_dialog_open = false;
        cx.notify();
    }

    /// Invite member
    fn invite_member(&mut self, cx: &mut Context<Self>) {
        if self.invite_email.trim().is_empty() {
            self.error_message = Some("Email is required".to_string());
            cx.notify();
            return;
        }

        if let Some(team) = self.selected_team() {
            cx.emit(TeamPanelEvent::InviteMember {
                team_id: team.id.clone(),
                email: self.invite_email.clone(),
                role: self.invite_role,
                message: if self.invite_message.is_empty() {
                    None
                } else {
                    Some(self.invite_message.clone())
                },
            });
        }

        self.close_invite_dialog(cx);
    }

    /// Set view mode (public for command palette integration)
    pub fn set_view_mode(&mut self, mode: TeamViewMode, cx: &mut Context<Self>) {
        self.view_mode = mode;
        cx.notify();
    }
}
