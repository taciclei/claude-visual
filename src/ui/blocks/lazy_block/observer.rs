//! Visibility observer for lazy loading

use super::types::BlockPosition;

/// Visibility observer for lazy loading
///
/// Tracks which blocks are visible in a scrollable container
pub struct VisibilityObserver {
    /// Registered blocks and their positions
    blocks: Vec<BlockPosition>,
    /// Current scroll position
    scroll_offset: f32,
    /// Viewport height
    viewport_height: f32,
    /// Preload margin
    preload_margin: f32,
}

impl Default for VisibilityObserver {
    fn default() -> Self {
        Self::new(500.0, 200.0)
    }
}

impl VisibilityObserver {
    /// Create a new observer
    pub fn new(viewport_height: f32, preload_margin: f32) -> Self {
        Self {
            blocks: Vec::new(),
            scroll_offset: 0.0,
            viewport_height,
            preload_margin,
        }
    }

    /// Register a block
    pub fn register_block(&mut self, id: impl Into<String>, top: f32, height: f32) {
        let id = id.into();

        // Remove existing if any
        self.blocks.retain(|b| b.id != id);

        self.blocks.push(BlockPosition {
            id,
            top,
            height,
            is_visible: false,
        });

        // Recalculate visibility
        self.update_visibility();
    }

    /// Unregister a block
    pub fn unregister_block(&mut self, id: &str) {
        self.blocks.retain(|b| b.id != id);
    }

    /// Update scroll position
    pub fn set_scroll_offset(&mut self, offset: f32) -> Vec<(String, bool)> {
        self.scroll_offset = offset;
        self.update_visibility()
    }

    /// Update viewport height
    pub fn set_viewport_height(&mut self, height: f32) -> Vec<(String, bool)> {
        self.viewport_height = height;
        self.update_visibility()
    }

    /// Recalculate visibility for all blocks
    fn update_visibility(&mut self) -> Vec<(String, bool)> {
        let mut changes = Vec::new();
        let visible_start = self.scroll_offset - self.preload_margin;
        let visible_end = self.scroll_offset + self.viewport_height + self.preload_margin;

        for block in &mut self.blocks {
            let block_end = block.top + block.height;
            let was_visible = block.is_visible;

            // Check if block intersects visible area
            block.is_visible = block.top < visible_end && block_end > visible_start;

            if block.is_visible != was_visible {
                changes.push((block.id.clone(), block.is_visible));
            }
        }

        changes
    }

    /// Get all visible block IDs
    pub fn visible_blocks(&self) -> Vec<&str> {
        self.blocks
            .iter()
            .filter(|b| b.is_visible)
            .map(|b| b.id.as_str())
            .collect()
    }

    /// Get all pending (not visible but registered) block IDs
    pub fn pending_blocks(&self) -> Vec<&str> {
        self.blocks
            .iter()
            .filter(|b| !b.is_visible)
            .map(|b| b.id.as_str())
            .collect()
    }

    /// Clear all blocks
    pub fn clear(&mut self) {
        self.blocks.clear();
    }
}
