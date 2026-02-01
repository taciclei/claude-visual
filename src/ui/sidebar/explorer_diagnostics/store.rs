//! Diagnostics store for the explorer

use crate::lsp::protocol::Diagnostic;
use crate::ui::sidebar::explorer_diagnostics::{DiagnosticCounts, ExplorerDiagnosticsConfig};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Diagnostics store for the explorer
pub struct ExplorerDiagnosticsStore {
    /// File path -> diagnostics
    file_diagnostics: HashMap<PathBuf, Vec<Diagnostic>>,
    /// Cached counts per path (files and directories)
    cached_counts: HashMap<PathBuf, DiagnosticCounts>,
    /// Configuration
    config: ExplorerDiagnosticsConfig,
    /// Dirty flag for cache
    cache_dirty: bool,
}

impl Default for ExplorerDiagnosticsStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ExplorerDiagnosticsStore {
    /// Create new store
    pub fn new() -> Self {
        Self {
            file_diagnostics: HashMap::new(),
            cached_counts: HashMap::new(),
            config: ExplorerDiagnosticsConfig::default(),
            cache_dirty: false,
        }
    }

    /// Create with custom config
    pub fn with_config(config: ExplorerDiagnosticsConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    /// Get configuration
    pub fn config(&self) -> &ExplorerDiagnosticsConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: ExplorerDiagnosticsConfig) {
        self.config = config;
        self.cache_dirty = true;
    }

    /// Update diagnostics for a file
    pub fn update_file(&mut self, path: PathBuf, diagnostics: Vec<Diagnostic>) {
        if diagnostics.is_empty() {
            self.file_diagnostics.remove(&path);
        } else {
            self.file_diagnostics.insert(path, diagnostics);
        }
        self.cache_dirty = true;
    }

    /// Clear diagnostics for a file
    pub fn clear_file(&mut self, path: &Path) {
        self.file_diagnostics.remove(path);
        self.cache_dirty = true;
    }

    /// Clear all diagnostics
    pub fn clear_all(&mut self) {
        self.file_diagnostics.clear();
        self.cached_counts.clear();
        self.cache_dirty = false;
    }

    /// Get diagnostics for a file
    pub fn get_file_diagnostics(&self, path: &Path) -> Option<&[Diagnostic]> {
        self.file_diagnostics.get(path).map(|v| v.as_slice())
    }

    /// Get diagnostic counts for a path (file or directory)
    pub fn get_counts(&mut self, path: &Path) -> DiagnosticCounts {
        if self.cache_dirty {
            self.rebuild_cache();
        }

        self.cached_counts.get(path).cloned().unwrap_or_default()
    }

    /// Rebuild the counts cache
    fn rebuild_cache(&mut self) {
        self.cached_counts.clear();

        for (path, diagnostics) in &self.file_diagnostics {
            let counts = DiagnosticCounts::from_diagnostics(diagnostics);
            self.cached_counts.insert(path.clone(), counts.clone());

            // Aggregate to parent directories if enabled
            if self.config.aggregate_to_parents {
                let mut current = path.parent();
                let mut depth = 0;

                while let Some(parent) = current {
                    if self.config.aggregation_depth >= 0 && depth >= self.config.aggregation_depth
                    {
                        break;
                    }

                    self.cached_counts
                        .entry(parent.to_path_buf())
                        .or_default()
                        .merge(&counts);

                    current = parent.parent();
                    depth += 1;
                }
            }
        }

        self.cache_dirty = false;
    }

    /// Get all files with diagnostics
    pub fn files_with_diagnostics(&self) -> impl Iterator<Item = &PathBuf> {
        self.file_diagnostics.keys()
    }

    /// Get total diagnostic counts across all files
    pub fn total_counts(&mut self) -> DiagnosticCounts {
        if self.cache_dirty {
            self.rebuild_cache();
        }

        let mut total = DiagnosticCounts::default();
        for (path, diagnostics) in &self.file_diagnostics {
            // Only count files, not aggregated directories
            if path.is_file() || !self.config.aggregate_to_parents {
                total.merge(&DiagnosticCounts::from_diagnostics(diagnostics));
            }
        }
        total
    }
}
