//! Per-project configuration

use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Project-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectConfig {
    /// Override theme for this project
    pub theme: Option<String>,
    /// Custom environment variables
    pub env: std::collections::HashMap<String, String>,
    /// Ignored patterns for file operations
    pub ignore_patterns: Vec<String>,
    /// Custom Claude CLI arguments
    pub claude_args: Vec<String>,
    /// Default branch for worktrees
    pub default_branch: Option<String>,
}

impl ProjectConfig {
    /// Config filename
    const FILENAME: &'static str = ".claude-visual.toml";

    /// Load config from a project directory
    pub fn load(project_path: &Path) -> Result<Self> {
        let config_path = project_path.join(Self::FILENAME);
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: ProjectConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// Save config to a project directory
    pub fn save(&self, project_path: &Path) -> Result<()> {
        let config_path = project_path.join(Self::FILENAME);
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    /// Check if config exists in a directory
    pub fn exists(project_path: &Path) -> bool {
        project_path.join(Self::FILENAME).exists()
    }

    /// Get the config file path
    pub fn config_path(project_path: &Path) -> PathBuf {
        project_path.join(Self::FILENAME)
    }
}
