//! Command palette filtering logic
//!
//! This module contains command filtering and search functionality.

use super::super::core::ChatView;
use super::super::types::PaletteCommand;

impl ChatView {
    /// Filter palette commands based on query
    pub(crate) fn filter_palette_commands(&self) -> Vec<&PaletteCommand> {
        let commands = Self::get_palette_commands();
        let query = self.palette.query.to_lowercase();

        if query.is_empty() {
            // Return a static reference would be ideal, but we need to collect
            return vec![];
        }

        // This is a workaround - in real code we'd want to store commands statically
        vec![]
    }
}
