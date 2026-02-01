//! Query operations for checkpoints

use super::manager::RollbackManager;
use super::types::RollbackCheckpoint;

impl RollbackManager {
    /// Get all checkpoints
    pub fn checkpoints(&self) -> &[RollbackCheckpoint] {
        &self.checkpoints
    }

    /// Get a checkpoint by ID
    pub fn get_checkpoint(&self, id: &str) -> Option<&RollbackCheckpoint> {
        self.checkpoints.iter().find(|c| c.id == id)
    }

    /// Get checkpoints for a specific step
    pub fn checkpoints_for_step(&self, step_number: usize) -> Vec<&RollbackCheckpoint> {
        self.checkpoints
            .iter()
            .filter(|c| c.step_number == Some(step_number))
            .collect()
    }
}
