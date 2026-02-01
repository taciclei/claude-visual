//! Data types for MCP tool execution progress tracking

use std::time::{Duration, Instant};

/// Status of an active tool execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionPhase {
    /// Preparing to execute
    Preparing,
    /// Executing the tool
    Executing,
    /// Processing results
    Processing,
    /// Completed successfully
    Completed,
    /// Failed with error
    Failed,
    /// Cancelled by user
    Cancelled,
}

impl ExecutionPhase {
    /// Get display text
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Preparing => "Preparing...",
            Self::Executing => "Executing...",
            Self::Processing => "Processing...",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
            Self::Cancelled => "Cancelled",
        }
    }

    /// Check if the execution is still active
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Preparing | Self::Executing | Self::Processing)
    }
}

/// An active tool execution
#[derive(Debug, Clone)]
pub struct ActiveExecution {
    /// Unique ID
    pub(crate) id: String,
    /// Tool name
    pub(crate) tool_name: String,
    /// Server name
    pub(crate) server_name: String,
    /// Current phase
    pub(crate) phase: ExecutionPhase,
    /// Start time
    pub(crate) started_at: Instant,
    /// Progress percentage (0-100), if available
    pub(crate) progress: Option<u8>,
    /// Current status message
    pub(crate) status_message: Option<String>,
    /// Error message if failed
    pub(crate) error: Option<String>,
}

impl ActiveExecution {
    /// Create a new active execution
    pub fn new(id: String, tool_name: String, server_name: String) -> Self {
        Self {
            id,
            tool_name,
            server_name,
            phase: ExecutionPhase::Preparing,
            started_at: Instant::now(),
            progress: None,
            status_message: None,
            error: None,
        }
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    /// Format elapsed time for display
    pub fn elapsed_str(&self) -> String {
        let elapsed = self.elapsed();
        let secs = elapsed.as_secs();
        if secs < 60 {
            format!("{}s", secs)
        } else {
            format!("{}m {}s", secs / 60, secs % 60)
        }
    }
}

/// Events emitted by the progress panel
pub enum ToolProgressPanelEvent {
    /// Cancel an execution
    Cancel(String),
    /// View execution details
    ViewDetails(String),
    /// Dismiss completed/failed execution
    Dismiss(String),
    /// Dismiss all completed
    DismissAll,
}
