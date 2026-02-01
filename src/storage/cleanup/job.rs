//! Cleanup job execution

use super::types::{CleanupConfig, CleanupItem, CleanupStats};

/// Cleanup job
pub struct CleanupJob {
    /// Configuration
    pub(crate) config: CleanupConfig,
    /// Items to clean
    pub(crate) items: Vec<CleanupItem>,
    /// Dry run mode
    pub(crate) dry_run: bool,
    /// Statistics
    pub(crate) stats: CleanupStats,
}

impl CleanupJob {
    /// Create a new cleanup job
    pub fn new(config: CleanupConfig) -> Self {
        Self {
            config,
            items: Vec::new(),
            dry_run: false,
            stats: CleanupStats::new(),
        }
    }

    /// Enable dry run mode
    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    /// Add items to clean
    pub fn add_items(&mut self, items: impl IntoIterator<Item = CleanupItem>) {
        self.items.extend(items);
    }

    /// Add a single item
    pub fn add_item(&mut self, item: CleanupItem) {
        self.items.push(item);
    }

    /// Get items that would be cleaned
    pub fn preview(&self) -> Vec<&CleanupItem> {
        self.items
            .iter()
            .filter(|item| item.should_cleanup(&self.config))
            .collect()
    }

    /// Calculate space that would be freed
    pub fn estimate_space_freed(&self) -> u64 {
        self.preview()
            .iter()
            .map(|item| item.size_bytes)
            .sum()
    }

    /// Execute the cleanup
    pub fn execute<F>(&mut self, mut delete_fn: F) -> &CleanupStats
    where
        F: FnMut(&CleanupItem) -> Result<(), String>,
    {
        let start = std::time::Instant::now();
        self.stats = CleanupStats::new();
        self.stats.items_scanned = self.items.len();

        for item in &self.items {
            if item.is_protected {
                self.stats.items_skipped += 1;
                continue;
            }

            if !item.should_cleanup(&self.config) {
                continue;
            }

            if self.dry_run {
                self.stats.items_cleaned += 1;
                self.stats.space_freed_bytes += item.size_bytes;
                continue;
            }

            match delete_fn(item) {
                Ok(()) => {
                    self.stats.items_cleaned += 1;
                    self.stats.space_freed_bytes += item.size_bytes;
                }
                Err(_) => {
                    self.stats.errors += 1;
                }
            }
        }

        self.stats.duration_ms = start.elapsed().as_millis() as u64;
        &self.stats
    }

    /// Get statistics
    pub fn stats(&self) -> &CleanupStats {
        &self.stats
    }
}
