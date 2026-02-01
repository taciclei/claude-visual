//! File system API for extensions

use parking_lot::RwLock;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::types::ApiResult;

/// File system API for extensions
pub struct FileSystemApi {
    /// Allowed paths for this extension
    allowed_paths: Arc<RwLock<Vec<PathBuf>>>,
}

impl Default for FileSystemApi {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystemApi {
    /// Create a new file system API instance
    pub fn new() -> Self {
        Self {
            allowed_paths: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add an allowed path for the extension
    pub fn allow_path(&self, path: PathBuf) {
        self.allowed_paths.write().push(path);
    }

    /// Check if a path is allowed
    pub fn is_path_allowed(&self, path: &Path) -> bool {
        let allowed = self.allowed_paths.read();
        allowed.iter().any(|allowed| path.starts_with(allowed))
    }

    /// Read a file (with permission check)
    pub fn read_file(&self, path: &Path) -> ApiResult {
        if !self.is_path_allowed(path) {
            return ApiResult::error("Access denied: path not in allowed list");
        }

        match std::fs::read_to_string(path) {
            Ok(content) => ApiResult::data(content),
            Err(e) => ApiResult::error(format!("Failed to read file: {}", e)),
        }
    }

    /// List directory contents (with permission check)
    pub fn list_directory(&self, path: &Path) -> ApiResult {
        if !self.is_path_allowed(path) {
            return ApiResult::error("Access denied: path not in allowed list");
        }

        match std::fs::read_dir(path) {
            Ok(entries) => {
                let names: Vec<String> = entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .collect();
                ApiResult::data(names.join("\n"))
            }
            Err(e) => ApiResult::error(format!("Failed to list directory: {}", e)),
        }
    }

    /// Check if a path exists (with permission check)
    pub fn path_exists(&self, path: &Path) -> ApiResult {
        if !self.is_path_allowed(path) {
            return ApiResult::error("Access denied: path not in allowed list");
        }

        ApiResult::data(path.exists().to_string())
    }
}
