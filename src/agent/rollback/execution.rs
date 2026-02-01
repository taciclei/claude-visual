//! Rollback execution logic

use std::time::Instant;

use super::manager::RollbackManager;
use super::types::{RollbackOperation, RollbackResult};

impl RollbackManager {
    /// Rollback to a specific checkpoint
    pub fn rollback_to(&mut self, checkpoint_id: &str) -> Result<RollbackResult, String> {
        // Find the checkpoint index
        let idx = self
            .checkpoints
            .iter()
            .position(|c| c.id == checkpoint_id)
            .ok_or_else(|| "Checkpoint not found".to_string())?;

        let start = Instant::now();
        let mut operations_rolled_back = 0;
        let mut failed_operations = Vec::new();

        // Rollback all checkpoints from the end to this one (inclusive)
        while self.checkpoints.len() > idx {
            let checkpoint = self.checkpoints.pop().unwrap();

            // Rollback operations in reverse order
            for op in checkpoint.operations().iter().rev() {
                match self.execute_rollback(op) {
                    Ok(()) => operations_rolled_back += 1,
                    Err(e) => failed_operations.push((op.clone(), e)),
                }
            }
        }

        let result = RollbackResult {
            success: failed_operations.is_empty(),
            operations_rolled_back,
            failed_operations,
            duration_ms: start.elapsed().as_millis() as u64,
        };

        Ok(result)
    }

    /// Rollback the last N checkpoints
    pub fn rollback_last(&mut self, count: usize) -> Result<RollbackResult, String> {
        if count == 0 {
            return Ok(RollbackResult {
                success: true,
                operations_rolled_back: 0,
                failed_operations: Vec::new(),
                duration_ms: 0,
            });
        }

        let target_len = self.checkpoints.len().saturating_sub(count);

        if self.checkpoints.len() <= target_len {
            return Err("Not enough checkpoints to rollback".to_string());
        }

        // Get the checkpoint ID to rollback to
        let checkpoint_id = if target_len > 0 {
            self.checkpoints[target_len - 1].id.clone()
        } else if !self.checkpoints.is_empty() {
            self.checkpoints[0].id.clone()
        } else {
            return Err("No checkpoints available".to_string());
        };

        self.rollback_to(&checkpoint_id)
    }

    /// Rollback a specific step
    pub fn rollback_step(&mut self, step_number: usize) -> Result<RollbackResult, String> {
        let checkpoint_ids: Vec<_> = self
            .checkpoints
            .iter()
            .filter(|c| c.step_number == Some(step_number))
            .map(|c| c.id.clone())
            .collect();

        if checkpoint_ids.is_empty() {
            return Err(format!("No checkpoints found for step {}", step_number));
        }

        let start = Instant::now();
        let mut total_rolled_back = 0;
        let mut all_failed = Vec::new();

        // Rollback each checkpoint for this step (in reverse order)
        for id in checkpoint_ids.into_iter().rev() {
            let result = self.rollback_to(&id)?;
            total_rolled_back += result.operations_rolled_back;
            all_failed.extend(result.failed_operations);
        }

        Ok(RollbackResult {
            success: all_failed.is_empty(),
            operations_rolled_back: total_rolled_back,
            failed_operations: all_failed,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Execute a single rollback operation
    pub(crate) fn execute_rollback(&self, op: &RollbackOperation) -> Result<(), String> {
        match op {
            RollbackOperation::FileCreated { path } => {
                std::fs::remove_file(path)
                    .map_err(|e| format!("Failed to delete file {}: {}", path.display(), e))
            }

            RollbackOperation::FileModified {
                path,
                original_content,
                original_permissions,
            } => {
                std::fs::write(path, original_content)
                    .map_err(|e| format!("Failed to restore file {}: {}", path.display(), e))?;

                #[cfg(unix)]
                if let Some(mode) = original_permissions {
                    use std::os::unix::fs::PermissionsExt;
                    std::fs::set_permissions(path, std::fs::Permissions::from_mode(*mode))
                        .map_err(|e| format!("Failed to restore permissions: {}", e))?;
                }

                Ok(())
            }

            RollbackOperation::FileDeleted {
                path,
                original_content,
                original_permissions,
            } => {
                // Create parent directories if needed
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create parent directory: {}", e))?;
                }

                std::fs::write(path, original_content)
                    .map_err(|e| format!("Failed to restore file {}: {}", path.display(), e))?;

                #[cfg(unix)]
                if let Some(mode) = original_permissions {
                    use std::os::unix::fs::PermissionsExt;
                    std::fs::set_permissions(path, std::fs::Permissions::from_mode(*mode))
                        .map_err(|e| format!("Failed to restore permissions: {}", e))?;
                }

                Ok(())
            }

            RollbackOperation::FileRenamed { from, to } => {
                std::fs::rename(to, from)
                    .map_err(|e| format!("Failed to rename {} back to {}: {}", to.display(), from.display(), e))
            }

            RollbackOperation::DirectoryCreated { path } => {
                std::fs::remove_dir_all(path)
                    .map_err(|e| format!("Failed to delete directory {}: {}", path.display(), e))
            }

            RollbackOperation::DirectoryDeleted { path, contents } => {
                // First create the directory
                std::fs::create_dir_all(path)
                    .map_err(|e| format!("Failed to create directory {}: {}", path.display(), e))?;

                // Then restore contents
                for content_op in contents {
                    self.execute_rollback(content_op)?;
                }

                Ok(())
            }

            RollbackOperation::CommandExecuted {
                rollback_command,
                cwd,
                ..
            } => {
                if let Some(cmd) = rollback_command {
                    let output = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(cmd)
                        .current_dir(cwd)
                        .output()
                        .map_err(|e| format!("Failed to execute rollback command: {}", e))?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(format!("Rollback command failed: {}", stderr));
                    }
                    Ok(())
                } else {
                    Err("No rollback command provided".to_string())
                }
            }

            RollbackOperation::GitCommit {
                repo_path,
                commit_hash,
            } => {
                // Revert the commit
                let output = std::process::Command::new("git")
                    .args(["revert", "--no-commit", commit_hash])
                    .current_dir(repo_path)
                    .output()
                    .map_err(|e| format!("Failed to revert commit: {}", e))?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Git revert failed: {}", stderr));
                }
                Ok(())
            }

            RollbackOperation::GitBranchCreated {
                repo_path,
                branch_name,
            } => {
                let output = std::process::Command::new("git")
                    .args(["branch", "-D", branch_name])
                    .current_dir(repo_path)
                    .output()
                    .map_err(|e| format!("Failed to delete branch: {}", e))?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Git branch delete failed: {}", stderr));
                }
                Ok(())
            }

            RollbackOperation::DatabaseInsert { .. } => {
                // Database operations need to be handled by the caller
                // with access to the database connection
                Err("Database rollback requires external handler".to_string())
            }

            RollbackOperation::DatabaseUpdate { .. } => {
                Err("Database rollback requires external handler".to_string())
            }

            RollbackOperation::DatabaseDelete { .. } => {
                Err("Database rollback requires external handler".to_string())
            }

            RollbackOperation::Custom { name, data } => {
                if let Some(handler) = self.custom_handlers.get(name) {
                    handler(data)
                } else {
                    Err(format!("No handler registered for custom operation: {}", name))
                }
            }
        }
    }
}
