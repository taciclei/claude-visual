//! Public getter methods

use super::state::AgentWorkspace;
use super::types::*;

impl AgentWorkspace {
    /// Get current mode
    pub fn mode(&self) -> AgentMode {
        self.mode
    }

    /// Check if agent is active
    pub fn is_active(&self) -> bool {
        matches!(
            self.mode,
            AgentMode::Planning | AgentMode::Executing | AgentMode::Paused
        )
    }

    /// Get progress
    pub fn progress(&self) -> f32 {
        self.progress
    }

    /// Get current step
    pub fn current_step(&self) -> usize {
        self.current_step
    }

    /// Get total steps
    pub fn total_steps(&self) -> usize {
        self.total_steps
    }
}
