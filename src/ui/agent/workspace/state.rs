//! Agent workspace state definition

use super::types::*;
use crate::agent::{Plan, TaskTree};
use gpui::*;

/// Agent workspace state
pub struct AgentWorkspace {
    /// Current mode
    pub(crate) mode: AgentMode,
    /// Current layout
    pub(crate) layout: AgentLayout,
    /// Settings
    pub(crate) settings: AgentSettings,
    /// Current task description
    pub(crate) task_description: Option<String>,
    /// Current plan
    pub(crate) plan: Option<Plan>,
    /// Task tree
    pub(crate) task_tree: Option<TaskTree>,
    /// Current step index
    pub(crate) current_step: usize,
    /// Total steps
    pub(crate) total_steps: usize,
    /// Pending approval step
    pub(crate) pending_approval: Option<PendingApproval>,
    /// Error message
    pub(crate) error: Option<String>,
    /// Progress percentage
    pub(crate) progress: f32,
    /// Elapsed time in seconds
    pub(crate) elapsed_seconds: u64,
    /// Log entries
    pub(crate) logs: Vec<LogEntry>,
    /// Max log entries
    pub(crate) max_logs: usize,
    /// Whether panel is expanded
    pub(crate) is_expanded: bool,
}

impl AgentWorkspace {
    /// Create new agent workspace
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            mode: AgentMode::Disabled,
            layout: AgentLayout::Compact,
            settings: AgentSettings::default(),
            task_description: None,
            plan: None,
            task_tree: None,
            current_step: 0,
            total_steps: 0,
            pending_approval: None,
            error: None,
            progress: 0.0,
            elapsed_seconds: 0,
            logs: Vec::new(),
            max_logs: 100,
            is_expanded: false,
        }
    }
}

impl EventEmitter<AgentWorkspaceEvent> for AgentWorkspace {}
