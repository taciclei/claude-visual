//! Agent workspace lifecycle methods

use super::state::AgentWorkspace;
use super::types::*;
use gpui::*;

impl AgentWorkspace {
    /// Enable agent mode
    pub fn enable(&mut self, cx: &mut Context<Self>) {
        if self.mode == AgentMode::Disabled {
            self.mode = AgentMode::Idle;
            self.add_log(LogLevel::Info, "Agent mode enabled");
            cx.emit(AgentWorkspaceEvent::ModeChanged(self.mode));
            cx.notify();
        }
    }

    /// Disable agent mode
    pub fn disable(&mut self, cx: &mut Context<Self>) {
        self.mode = AgentMode::Disabled;
        self.reset();
        self.add_log(LogLevel::Info, "Agent mode disabled");
        cx.emit(AgentWorkspaceEvent::ModeChanged(self.mode));
        cx.notify();
    }
}
