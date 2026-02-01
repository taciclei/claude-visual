//! Event handlers for project sidebar

use std::path::PathBuf;

use gpui::prelude::*;
use gpui::*;

use crate::project::manager::Project;

use super::types::{ProjectsSidebar, ProjectsSidebarEvent};

impl ProjectsSidebar {
    /// Refresh project list
    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        self.projects = self
            .app_state
            .project_manager
            .read(cx)
            .list_projects()
            .unwrap_or_default();
        cx.notify();
    }

    /// Select a project by index
    pub fn select_project(&mut self, index: usize, cx: &mut Context<Self>) {
        // Find the actual project from filtered list
        let filtered = self.filtered_projects();
        if let Some(project) = filtered.get(index) {
            let id = project.id.clone();
            let path = project.path.clone();

            // Find the actual index in the original projects list
            if let Some(actual_idx) = self.projects.iter().position(|p| p.path == path) {
                self.selected_project = Some(actual_idx);
            }

            cx.emit(ProjectsSidebarEvent::ProjectSelected(id, path));
            cx.notify();
        }
    }

    /// Update filter text
    pub fn set_filter(&mut self, text: String, cx: &mut Context<Self>) {
        self.filter_text = text;
        cx.notify();
    }

    /// Handle search key input
    pub(super) fn handle_search_key(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match &event.keystroke.key {
            key if key == "backspace" => {
                self.filter_text.pop();
                cx.notify();
            }
            key if key == "escape" => {
                self.filter_text.clear();
                cx.notify();
            }
            _ => {}
        }
    }

    /// Handle search text input
    pub(super) fn handle_search_input(
        &mut self,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.filter_text.push_str(text);
        cx.notify();
    }

    /// Filter projects
    pub(super) fn filtered_projects(&self) -> Vec<&Project> {
        if self.filter_text.is_empty() {
            self.projects.iter().collect()
        } else {
            let filter = self.filter_text.to_lowercase();
            self.projects
                .iter()
                .filter(|p| p.name.to_lowercase().contains(&filter))
                .collect()
        }
    }

    /// Handle file drop
    pub(super) fn handle_file_drop(
        &mut self,
        paths: &ExternalPaths,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_drag_over = false;

        // Filter to only directories
        let directories: Vec<PathBuf> = paths
            .paths()
            .iter()
            .filter(|p| p.is_dir())
            .cloned()
            .collect();

        if !directories.is_empty() {
            cx.emit(ProjectsSidebarEvent::FilesDropped(directories));
        }

        cx.notify();
    }

    /// Set drag over state
    pub(super) fn set_drag_over(&mut self, is_over: bool, cx: &mut Context<Self>) {
        if self.is_drag_over != is_over {
            self.is_drag_over = is_over;
            cx.notify();
        }
    }
}
