//! Core state management for tool progress panel

use std::collections::HashMap;
use std::sync::Arc;

use gpui::*;

use super::types::{ActiveExecution, ExecutionPhase};
use crate::app::state::AppState;

/// Panel for displaying active tool executions
pub struct ToolProgressPanel {
    pub(crate) app_state: Arc<AppState>,
    /// Active executions by ID
    pub(crate) executions: HashMap<String, ActiveExecution>,
    /// Order of execution IDs for display
    pub(crate) execution_order: Vec<String>,
    /// Whether panel is collapsed
    pub(crate) collapsed: bool,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}

impl ToolProgressPanel {
    /// Create a new progress panel
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            executions: HashMap::new(),
            execution_order: Vec::new(),
            collapsed: false,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Start tracking a new execution
    pub fn start_execution(
        &mut self,
        id: String,
        tool_name: String,
        server_name: String,
        cx: &mut Context<Self>,
    ) {
        let execution = ActiveExecution::new(id.clone(), tool_name, server_name);
        self.executions.insert(id.clone(), execution);
        self.execution_order.push(id);
        cx.notify();
    }

    /// Update execution phase
    pub fn update_phase(&mut self, id: &str, phase: ExecutionPhase, cx: &mut Context<Self>) {
        if let Some(execution) = self.executions.get_mut(id) {
            execution.phase = phase;
            cx.notify();
        }
    }

    /// Update execution progress
    pub fn update_progress(
        &mut self,
        id: &str,
        progress: Option<u8>,
        message: Option<String>,
        cx: &mut Context<Self>,
    ) {
        if let Some(execution) = self.executions.get_mut(id) {
            execution.progress = progress;
            execution.status_message = message;
            cx.notify();
        }
    }

    /// Mark execution as completed
    pub fn complete_execution(&mut self, id: &str, cx: &mut Context<Self>) {
        if let Some(execution) = self.executions.get_mut(id) {
            execution.phase = ExecutionPhase::Completed;
            cx.notify();
        }
    }

    /// Mark execution as failed
    pub fn fail_execution(&mut self, id: &str, error: String, cx: &mut Context<Self>) {
        if let Some(execution) = self.executions.get_mut(id) {
            execution.phase = ExecutionPhase::Failed;
            execution.error = Some(error);
            cx.notify();
        }
    }

    /// Remove an execution
    pub fn remove_execution(&mut self, id: &str, cx: &mut Context<Self>) {
        self.executions.remove(id);
        self.execution_order.retain(|i| i != id);
        cx.notify();
    }

    /// Remove all completed/failed executions
    pub fn remove_completed(&mut self, cx: &mut Context<Self>) {
        let to_remove: Vec<_> = self
            .executions
            .iter()
            .filter(|(_, e)| !e.phase.is_active())
            .map(|(id, _)| id.clone())
            .collect();

        for id in to_remove {
            self.executions.remove(&id);
            self.execution_order.retain(|i| i != &id);
        }
        cx.notify();
    }

    /// Get count of active executions
    pub fn active_count(&self) -> usize {
        self.executions
            .values()
            .filter(|e| e.phase.is_active())
            .count()
    }

    /// Check if there are any executions to show
    pub fn has_executions(&self) -> bool {
        !self.executions.is_empty()
    }

    /// Toggle collapsed state
    pub(crate) fn toggle_collapsed(&mut self, cx: &mut Context<Self>) {
        self.collapsed = !self.collapsed;
        cx.notify();
    }
}
