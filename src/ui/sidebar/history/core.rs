//! Core HistorySidebar struct and basic methods

use std::sync::Arc;

use gpui::prelude::*;
use gpui::FocusHandle;

use crate::app::state::AppState;
use crate::project::manager::Project;
use crate::storage::models::{Conversation, SearchFilter, SearchResult};

use super::types::*;

/// History sidebar showing recent conversations
pub struct HistorySidebar {
    pub(crate) app_state: Arc<AppState>,
    pub(crate) conversations: Vec<Conversation>,
    pub(crate) selected_conversation: Option<String>,
    pub(crate) search_query: String,
    pub(crate) search_results: Vec<SearchResult>,
    pub(crate) display_mode: DisplayMode,
    pub(crate) focus_handle: FocusHandle,
    pub(crate) search_focus_handle: FocusHandle,
    pub(crate) search_filter: SearchFilter,
    pub(crate) show_filters: bool,
    pub(crate) projects: Vec<Project>,
}

impl HistorySidebar {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        // Load conversations from database
        let conversations = app_state
            .database
            .get_conversations(None)
            .unwrap_or_default();

        // Load projects
        let projects = app_state
            .project_manager
            .read(cx)
            .list_projects()
            .unwrap_or_default();

        Self {
            app_state,
            conversations,
            selected_conversation: None,
            search_query: String::new(),
            search_results: Vec::new(),
            display_mode: DisplayMode::Recent,
            focus_handle: cx.focus_handle(),
            search_focus_handle: cx.focus_handle(),
            search_filter: SearchFilter::default(),
            show_filters: false,
            projects,
        }
    }

    /// Refresh conversation list
    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        self.conversations = self
            .app_state
            .database
            .get_conversations(None)
            .unwrap_or_default();
        cx.notify();
    }

    /// Select a conversation
    pub fn select_conversation(&mut self, id: &str, cx: &mut Context<Self>) {
        self.selected_conversation = Some(id.to_string());
        cx.emit(HistorySidebarEvent::ConversationSelected(id.to_string()));
        cx.notify();
    }

    /// Delete a conversation
    pub fn delete_conversation(&mut self, id: &str, cx: &mut Context<Self>) {
        if let Err(e) = self.app_state.database.delete_conversation(id) {
            tracing::error!("Failed to delete conversation: {}", e);
        } else {
            // If this was the selected conversation, clear selection
            if self.selected_conversation.as_deref() == Some(id) {
                self.selected_conversation = None;
            }
            self.refresh(cx);
        }
    }
}
