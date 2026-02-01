//! Execution control methods

use super::state::AgentWorkspace;
use super::types::*;
use gpui::*;

impl AgentWorkspace {
    /// Pause execution
    pub fn pause(&mut self, cx: &mut Context<Self>) {
        if self.mode == AgentMode::Executing {
            self.mode = AgentMode::Paused;
            self.add_log(LogLevel::Info, "Execution paused");
            cx.emit(AgentWorkspaceEvent::ModeChanged(self.mode));
            cx.notify();
        }
    }

    /// Resume execution
    pub fn resume(&mut self, cx: &mut Context<Self>) {
        if self.mode == AgentMode::Paused && self.pending_approval.is_none() {
            self.mode = AgentMode::Executing;
            self.add_log(LogLevel::Info, "Execution resumed");
            cx.emit(AgentWorkspaceEvent::ModeChanged(self.mode));
            cx.notify();
        }
    }

    /// Cancel the current task
    pub fn cancel(&mut self, cx: &mut Context<Self>) {
        let task_desc = self.task_description.clone().unwrap_or_default();
        self.add_log(LogLevel::Warning, "Task cancelled by user");
        self.mode = AgentMode::Idle;
        self.pending_approval = None;
        cx.emit(AgentWorkspaceEvent::TaskFailed(
            task_desc,
            "Cancelled by user".to_string(),
        ));
        cx.emit(AgentWorkspaceEvent::ModeChanged(self.mode));
        cx.notify();
    }
}
