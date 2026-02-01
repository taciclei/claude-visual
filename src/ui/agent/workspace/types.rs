//! Agent workspace types and enums

use gpui::*;

pub(crate) struct SimpleColors {
    pub(crate) surface: Hsla,
    pub(crate) background: Hsla,
    pub(crate) border: Hsla,
    pub(crate) hover: Hsla,
    pub(crate) text: Hsla,
    pub(crate) text_muted: Hsla,
    pub(crate) accent: Hsla,
    pub(crate) error: Hsla,
    pub(crate) success: Hsla,
    pub(crate) warning: Hsla,
    pub(crate) selection: Hsla,
}

pub(crate) fn default_colors() -> SimpleColors {
    SimpleColors {
        surface: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
        background: hsla(220.0 / 360.0, 0.13, 0.09, 1.0),
        border: hsla(220.0 / 360.0, 0.13, 0.20, 1.0),
        hover: hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
        text: hsla(0.0, 0.0, 0.93, 1.0),
        text_muted: hsla(0.0, 0.0, 0.60, 1.0),
        accent: hsla(210.0 / 360.0, 0.80, 0.55, 1.0),
        error: hsla(0.0, 0.84, 0.60, 1.0),
        success: hsla(142.0 / 360.0, 0.71, 0.45, 1.0),
        warning: hsla(38.0 / 360.0, 0.92, 0.50, 1.0),
        selection: hsla(210.0 / 360.0, 0.50, 0.30, 1.0),
    }
}

/// Agent workspace mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentMode {
    /// Normal chat mode (agent disabled)
    Disabled,
    /// Agent is idle, waiting for task
    Idle,
    /// Planning phase
    Planning,
    /// Executing plan
    Executing,
    /// Paused for approval or user input
    Paused,
    /// Task completed
    Completed,
    /// Task failed
    Failed,
}

/// Events emitted by agent workspace
#[derive(Debug, Clone)]
pub enum AgentWorkspaceEvent {
    /// Agent mode changed
    ModeChanged(AgentMode),
    /// New task started
    TaskStarted(String),
    /// Task completed
    TaskCompleted(String),
    /// Task failed
    TaskFailed(String, String),
    /// User approval required
    ApprovalRequired(String, String),
    /// Plan generated
    PlanGenerated(String),
    /// Step completed
    StepCompleted(usize, String),
    /// User input required
    UserInputRequired(String),
    /// Request to show notification
    ShowNotification(String, NotificationType),
}

/// Notification type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

/// Agent workspace panel layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentLayout {
    /// Compact - just status bar
    Compact,
    /// Normal - task panel + executor
    Normal,
    /// Expanded - full plan view
    Expanded,
}

/// Agent workspace settings
#[derive(Debug, Clone)]
pub struct AgentSettings {
    /// Auto-approve low-risk steps
    pub auto_approve_low_risk: bool,
    /// Show notifications for step completion
    pub notify_on_step_complete: bool,
    /// Pause on warnings
    pub pause_on_warning: bool,
    /// Max consecutive auto-approvals
    pub max_auto_approvals: usize,
    /// Enable sound notifications
    pub sound_notifications: bool,
}

impl Default for AgentSettings {
    fn default() -> Self {
        Self {
            auto_approve_low_risk: true,
            notify_on_step_complete: true,
            pause_on_warning: true,
            max_auto_approvals: 5,
            sound_notifications: false,
        }
    }
}

/// Pending approval info
#[derive(Debug, Clone)]
pub(crate) struct PendingApproval {
    pub(crate) step_index: usize,
    pub(crate) step_description: String,
    pub(crate) risk_level: String,
    pub(crate) tool_name: Option<String>,
}

/// Log entry
#[derive(Debug, Clone)]
pub(crate) struct LogEntry {
    pub(crate) timestamp: chrono::DateTime<chrono::Utc>,
    pub(crate) level: LogLevel,
    pub(crate) message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}
