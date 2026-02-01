//! Debug session types and data structures

use std::path::PathBuf;

/// Debug session state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DebugState {
    /// Not started
    #[default]
    Idle,
    /// Initializing adapter
    Initializing,
    /// Running (executing)
    Running,
    /// Stopped (at breakpoint, exception, etc.)
    Stopped,
    /// Paused by user
    Paused,
    /// Session terminated
    Terminated,
    /// Error state
    Error,
}

impl DebugState {
    /// Get icon for state
    pub fn icon(&self) -> &'static str {
        match self {
            DebugState::Idle => "○",
            DebugState::Initializing => "◐",
            DebugState::Running => "▶",
            DebugState::Stopped => "⏹",
            DebugState::Paused => "⏸",
            DebugState::Terminated => "⏹",
            DebugState::Error => "⚠",
        }
    }

    /// Get color for state (RGB)
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            DebugState::Idle => (128, 128, 128),
            DebugState::Initializing => (209, 154, 102),
            DebugState::Running => (98, 181, 67),
            DebugState::Stopped => (224, 108, 117),
            DebugState::Paused => (97, 175, 239),
            DebugState::Terminated => (128, 128, 128),
            DebugState::Error => (224, 108, 117),
        }
    }
}

/// Events emitted by debug session
#[derive(Debug, Clone)]
pub enum SessionEvent {
    /// State changed
    StateChanged(DebugState),
    /// Output received
    Output { category: String, text: String },
    /// Stopped at breakpoint or exception
    Stopped {
        reason: String,
        thread_id: Option<i64>,
        description: Option<String>,
    },
    /// Thread started
    ThreadStarted(i64),
    /// Thread exited
    ThreadExited(i64),
    /// Breakpoint changed
    BreakpointChanged(super::super::protocol::Breakpoint),
    /// Module loaded
    ModuleLoaded { name: String, path: Option<String> },
    /// Session terminated
    Terminated,
    /// Error occurred
    Error(String),
}

/// User-defined breakpoint
#[derive(Debug, Clone)]
pub struct UserBreakpoint {
    /// Unique ID
    pub id: usize,
    /// File path
    pub file: PathBuf,
    /// Line number (1-indexed)
    pub line: i64,
    /// Optional condition
    pub condition: Option<String>,
    /// Optional hit count
    pub hit_count: Option<i64>,
    /// Log message (logpoint)
    pub log_message: Option<String>,
    /// Is enabled
    pub enabled: bool,
    /// Verified by adapter
    pub verified: bool,
}

impl UserBreakpoint {
    pub fn new(file: PathBuf, line: i64) -> Self {
        Self {
            id: 0, // Will be set by session
            file,
            line,
            condition: None,
            hit_count: None,
            log_message: None,
            enabled: true,
            verified: false,
        }
    }

    pub fn with_condition(mut self, condition: &str) -> Self {
        self.condition = Some(condition.to_string());
        self
    }

    pub fn with_log_message(mut self, message: &str) -> Self {
        self.log_message = Some(message.to_string());
        self
    }
}
