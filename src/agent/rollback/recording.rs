//! Recording operations into checkpoints

use std::path::{Path, PathBuf};

use super::manager::RollbackManager;
use super::types::RollbackOperation;

impl RollbackManager {
    /// Record an operation in the current checkpoint
    pub fn record(&mut self, operation: RollbackOperation) -> Result<(), &'static str> {
        match &mut self.current {
            Some(checkpoint) => {
                checkpoint.add_operation(operation);
                Ok(())
            }
            None => Err("No active checkpoint"),
        }
    }

    /// Record a file creation
    pub fn record_file_created(&mut self, path: impl Into<PathBuf>) -> Result<(), &'static str> {
        self.record(RollbackOperation::FileCreated { path: path.into() })
    }

    /// Record a file modification (capturing original content)
    pub fn record_file_modified(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let original_content = std::fs::read(path)?;

        #[cfg(unix)]
        let original_permissions = {
            use std::os::unix::fs::PermissionsExt;
            Some(std::fs::metadata(path)?.permissions().mode())
        };
        #[cfg(not(unix))]
        let original_permissions = None;

        self.record(RollbackOperation::FileModified {
            path: path.to_path_buf(),
            original_content,
            original_permissions,
        })?;

        Ok(())
    }

    /// Record a file deletion (capturing content before deletion)
    pub fn record_file_deleted(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let original_content = std::fs::read(path)?;

        #[cfg(unix)]
        let original_permissions = {
            use std::os::unix::fs::PermissionsExt;
            Some(std::fs::metadata(path)?.permissions().mode())
        };
        #[cfg(not(unix))]
        let original_permissions = None;

        self.record(RollbackOperation::FileDeleted {
            path: path.to_path_buf(),
            original_content,
            original_permissions,
        })?;

        Ok(())
    }

    /// Record a command execution
    pub fn record_command(
        &mut self,
        command: impl Into<String>,
        args: Vec<String>,
        cwd: impl Into<PathBuf>,
        rollback_command: Option<String>,
    ) -> Result<(), &'static str> {
        self.record(RollbackOperation::CommandExecuted {
            command: command.into(),
            args,
            cwd: cwd.into(),
            rollback_command,
        })
    }
}
