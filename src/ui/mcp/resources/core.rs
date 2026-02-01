//! Core implementation of MCP resources panel

use gpui::*;
use std::sync::Arc;

use crate::app::state::AppState;
use crate::mcp::McpPrompt;
use crate::mcp::McpResource;

use super::types::{PromptItem, ResourceItem, ResourcesTab};

/// MCP Resources Panel for viewing resources and prompts
pub struct McpResourcesPanel {
    pub(crate) app_state: Arc<AppState>,
    /// Available resources from all connected servers
    pub(crate) resources: Vec<ResourceItem>,
    /// Available prompts from all connected servers
    pub(crate) prompts: Vec<PromptItem>,
    /// Active tab
    pub(crate) active_tab: ResourcesTab,
    /// Search filter
    pub(crate) filter_text: String,
    /// Selected resource/prompt index
    pub(crate) selected_index: Option<usize>,
    /// Whether panel is expanded
    pub(crate) expanded: bool,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}

impl McpResourcesPanel {
    /// Create a new MCP resources panel
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            resources: Vec::new(),
            prompts: Vec::new(),
            active_tab: ResourcesTab::Resources,
            filter_text: String::new(),
            selected_index: None,
            expanded: true,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Set resources
    pub fn set_resources(&mut self, resources: Vec<ResourceItem>, cx: &mut Context<Self>) {
        self.resources = resources;
        cx.notify();
    }

    /// Add a resource
    pub fn add_resource(&mut self, server: String, resource: McpResource, cx: &mut Context<Self>) {
        self.resources.push(ResourceItem { server, resource });
        cx.notify();
    }

    /// Remove all resources from a server
    pub fn remove_server_resources(&mut self, server: &str, cx: &mut Context<Self>) {
        self.resources.retain(|r| r.server != server);
        cx.notify();
    }

    /// Set prompts
    pub fn set_prompts(&mut self, prompts: Vec<PromptItem>, cx: &mut Context<Self>) {
        self.prompts = prompts;
        cx.notify();
    }

    /// Add a prompt
    pub fn add_prompt(&mut self, server: String, prompt: McpPrompt, cx: &mut Context<Self>) {
        self.prompts.push(PromptItem { server, prompt });
        cx.notify();
    }

    /// Remove all prompts from a server
    pub fn remove_server_prompts(&mut self, server: &str, cx: &mut Context<Self>) {
        self.prompts.retain(|p| p.server != server);
        cx.notify();
    }

    /// Switch tab
    pub fn set_tab(&mut self, tab: ResourcesTab, cx: &mut Context<Self>) {
        self.active_tab = tab;
        self.selected_index = None;
        cx.notify();
    }

    /// Set filter
    pub fn set_filter(&mut self, text: String, cx: &mut Context<Self>) {
        self.filter_text = text;
        cx.notify();
    }

    /// Toggle expanded
    pub fn toggle_expanded(&mut self, cx: &mut Context<Self>) {
        self.expanded = !self.expanded;
        cx.notify();
    }

    /// Get filtered resources
    pub(crate) fn filtered_resources(&self) -> Vec<&ResourceItem> {
        self.resources
            .iter()
            .filter(|r| {
                if self.filter_text.is_empty() {
                    true
                } else {
                    let filter = self.filter_text.to_lowercase();
                    r.resource.name.to_lowercase().contains(&filter)
                        || r.resource.uri.to_lowercase().contains(&filter)
                        || r.resource
                            .description
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&filter))
                            .unwrap_or(false)
                        || r.server.to_lowercase().contains(&filter)
                }
            })
            .collect()
    }

    /// Get filtered prompts
    pub(crate) fn filtered_prompts(&self) -> Vec<&PromptItem> {
        self.prompts
            .iter()
            .filter(|p| {
                if self.filter_text.is_empty() {
                    true
                } else {
                    let filter = self.filter_text.to_lowercase();
                    p.prompt.name.to_lowercase().contains(&filter)
                        || p.prompt
                            .description
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&filter))
                            .unwrap_or(false)
                        || p.server.to_lowercase().contains(&filter)
                }
            })
            .collect()
    }
}
