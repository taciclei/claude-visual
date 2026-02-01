//! Checkpoint lifecycle operations

use super::manager::RollbackManager;
use super::types::RollbackCheckpoint;

impl RollbackManager {
    /// Start a new checkpoint
    pub fn begin_checkpoint(&mut self, name: impl Into<String>) {
        self.current = Some(RollbackCheckpoint::new(name));
    }

    /// Start a checkpoint for a plan step
    pub fn begin_step_checkpoint(&mut self, step_number: usize, name: impl Into<String>) {
        self.current = Some(RollbackCheckpoint::for_step(step_number, name));
    }

    /// Commit the current checkpoint
    pub fn commit_checkpoint(&mut self) -> Option<String> {
        if let Some(checkpoint) = self.current.take() {
            if !checkpoint.is_empty() {
                let id = checkpoint.id.clone();
                self.checkpoints.push(checkpoint);

                // Trim old checkpoints
                while self.checkpoints.len() > self.max_checkpoints {
                    self.checkpoints.remove(0);
                }

                return Some(id);
            }
        }
        None
    }

    /// Discard the current checkpoint without committing
    pub fn discard_checkpoint(&mut self) {
        self.current = None;
    }
}
