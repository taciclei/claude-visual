//! Approval system for risky operations

use super::state::AgentWorkspace;
use super::types::*;
use gpui::*;

impl AgentWorkspace {
    /// Request approval for a step
    pub fn request_approval(
        &mut self,
        step_index: usize,
        description: impl Into<String>,
        risk_level: impl Into<String>,
        tool_name: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let description = description.into();
        let risk_level = risk_level.into();

        self.pending_approval = Some(PendingApproval {
            step_index,
            step_description: description.clone(),
            risk_level: risk_level.clone(),
            tool_name: tool_name.clone(),
        });
        self.mode = AgentMode::Paused;
        self.add_log(
            LogLevel::Warning,
            format!("Approval required: {} ({})", description, risk_level),
        );
        cx.emit(AgentWorkspaceEvent::ApprovalRequired(
            description,
            risk_level,
        ));
        cx.emit(AgentWorkspaceEvent::ModeChanged(self.mode));
        cx.notify();
    }

    /// Approve pending step
    pub fn approve(&mut self, cx: &mut Context<Self>) {
        if let Some(approval) = self.pending_approval.take() {
            self.add_log(
                LogLevel::Info,
                format!("Approved: {}", approval.step_description),
            );
            self.mode = AgentMode::Executing;
            cx.emit(AgentWorkspaceEvent::ModeChanged(self.mode));
            cx.notify();
        }
    }

    /// Reject pending step
    pub fn reject(&mut self, reason: impl Into<String>, cx: &mut Context<Self>) {
        let reason = reason.into();
        if let Some(approval) = self.pending_approval.take() {
            self.add_log(
                LogLevel::Warning,
                format!("Rejected: {} - {}", approval.step_description, reason),
            );
            self.mode = AgentMode::Paused;
            cx.notify();
        }
    }
}
