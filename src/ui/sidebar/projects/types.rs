//! Project sidebar types and state

use std::path::PathBuf;
use std::sync::Arc;

use gpui::*;

use crate::app::state::AppState;
use crate::project::manager::Project;

/// Events emitted by ProjectsSidebar
pub enum ProjectsSidebarEvent {
    /// A project was selected (project_id, path)
    ProjectSelected(String, PathBuf),
    /// User requested to add a new project
    AddProjectRequested,
    /// Files/folders were dropped onto the sidebar
    FilesDropped(Vec<PathBuf>),
    /// Send a Claude Code skill command
    SendSkillCommand(String),
}

impl EventEmitter<ProjectsSidebarEvent> for ProjectsSidebar {}

/// Projects sidebar showing project list
pub struct ProjectsSidebar {
    pub(super) app_state: Arc<AppState>,
    pub(super) projects: Vec<Project>,
    pub(super) selected_project: Option<usize>,
    pub(super) filter_text: String,
    pub(super) search_focus_handle: FocusHandle,
    /// Whether files are being dragged over this component
    pub(super) is_drag_over: bool,
}

impl ProjectsSidebar {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        // Load projects from database
        let projects = app_state
            .project_manager
            .read(cx)
            .list_projects()
            .unwrap_or_default();

        Self {
            app_state,
            projects,
            selected_project: None,
            filter_text: String::new(),
            search_focus_handle: cx.focus_handle(),
            is_drag_over: false,
        }
    }
}
