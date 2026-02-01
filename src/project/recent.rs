//! Recent projects tracking

use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Entry in the recent projects list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentEntry {
    pub path: PathBuf,
    pub name: String,
    pub timestamp: i64,
}

/// Recent projects tracker
pub struct RecentProjects {
    entries: Vec<RecentEntry>,
    max_entries: usize,
}

impl Default for RecentProjects {
    fn default() -> Self {
        Self::new(20)
    }
}

impl RecentProjects {
    /// Create a new recent projects tracker
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }

    /// Load from disk
    pub fn load() -> Result<Self> {
        let path = Self::storage_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let entries: Vec<RecentEntry> = serde_json::from_str(&content)?;
            Ok(Self {
                entries,
                max_entries: 20,
            })
        } else {
            Ok(Self::default())
        }
    }

    /// Save to disk
    pub fn save(&self) -> Result<()> {
        let path = Self::storage_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(&self.entries)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    /// Get storage path
    fn storage_path() -> Result<PathBuf> {
        let data_dir =
            dirs::data_dir().ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?;
        Ok(data_dir.join("claude-visual").join("recent.json"))
    }

    /// Add a path to recent
    pub fn add(&mut self, path: PathBuf) {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        // Remove existing entry for this path
        self.entries.retain(|e| e.path != path);

        // Add new entry at the front
        self.entries.insert(
            0,
            RecentEntry {
                path,
                name,
                timestamp: chrono::Utc::now().timestamp(),
            },
        );

        // Trim to max entries
        self.entries.truncate(self.max_entries);
    }

    /// Remove a path from recent
    pub fn remove(&mut self, path: &PathBuf) {
        self.entries.retain(|e| &e.path != path);
    }

    /// Get recent entries
    pub fn entries(&self) -> &[RecentEntry] {
        &self.entries
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
