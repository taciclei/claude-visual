//! Internal helper methods

use super::state::AgentWorkspace;
use super::types::*;

impl AgentWorkspace {
    /// Reset state
    pub(crate) fn reset(&mut self) {
        self.task_description = None;
        self.plan = None;
        self.task_tree = None;
        self.current_step = 0;
        self.total_steps = 0;
        self.pending_approval = None;
        self.error = None;
        self.progress = 0.0;
        self.elapsed_seconds = 0;
    }

    /// Update progress percentage
    pub(crate) fn update_progress(&mut self) {
        if self.total_steps > 0 {
            self.progress = (self.current_step as f32 / self.total_steps as f32) * 100.0;
        }
    }

    /// Add log entry
    pub(crate) fn add_log(&mut self, level: LogLevel, message: impl Into<String>) {
        self.logs.push(LogEntry {
            timestamp: chrono::Utc::now(),
            level,
            message: message.into(),
        });

        // Trim old logs
        while self.logs.len() > self.max_logs {
            self.logs.remove(0);
        }
    }
}
