//! Core logic for MCP server configuration editor

use std::sync::Arc;
use gpui::*;

use crate::app::state::AppState;
use crate::mcp::McpServerConfig;

use super::types::*;

impl EventEmitter<ServerConfigEditorEvent> for ServerConfigEditor {}

/// Server configuration editor panel
pub struct ServerConfigEditor {
    pub(crate) app_state: Arc<AppState>,
    /// Configuration being edited
    pub(crate) config: EditingServerConfig,
    /// Original name (for rename detection)
    pub(crate) original_name: Option<String>,
    /// Currently focused field
    pub(crate) focused_field: Option<EditingField>,
    /// Validation error
    pub(crate) error: Option<String>,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}

impl ServerConfigEditor {
    /// Create editor for new server
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            config: EditingServerConfig::new_server(),
            original_name: None,
            focused_field: Some(EditingField::Name),
            error: None,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Create editor for existing server
    pub fn edit(
        app_state: Arc<AppState>,
        name: String,
        config: &McpServerConfig,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            config: EditingServerConfig::from_config(name.clone(), config),
            original_name: Some(name),
            focused_field: None,
            error: None,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Get the server name
    pub fn name(&self) -> &str {
        &self.config.name
    }

    /// Check if this is a new server
    pub fn is_new(&self) -> bool {
        self.config.is_new
    }

    /// Update a text field
    pub(crate) fn update_field(&mut self, field: EditingField, value: String, cx: &mut Context<Self>) {
        match field {
            EditingField::Name => self.config.name = value,
            EditingField::Command => self.config.command = value,
            EditingField::Args => self.config.args = value,
            EditingField::Env => self.config.env = value,
            EditingField::Description => self.config.description = value,
            EditingField::AutoApprove => self.config.auto_approve = value,
        }
        self.error = None;
        cx.notify();
    }

    /// Toggle enabled state
    pub(crate) fn toggle_enabled(&mut self, cx: &mut Context<Self>) {
        self.config.enabled = !self.config.enabled;
        cx.notify();
    }

    /// Save configuration
    pub(crate) fn save(&mut self, cx: &mut Context<Self>) {
        // Validate
        if let Err(err) = self.config.validate() {
            self.error = Some(err);
            cx.notify();
            return;
        }

        cx.emit(ServerConfigEditorEvent::Save {
            original_name: self.original_name.clone(),
            name: self.config.name.clone(),
            config: self.config.to_config(),
        });
    }

    /// Cancel editing
    pub(crate) fn cancel(&mut self, cx: &mut Context<Self>) {
        cx.emit(ServerConfigEditorEvent::Cancel);
    }

    /// Delete server
    pub(crate) fn delete(&mut self, cx: &mut Context<Self>) {
        if let Some(name) = &self.original_name {
            cx.emit(ServerConfigEditorEvent::Delete(name.clone()));
        }
    }
}

impl Focusable for ServerConfigEditor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
