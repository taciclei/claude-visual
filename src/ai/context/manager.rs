//! Context manager for tracking attached context items

use super::types::{ContextError, ContextItem};
use regex::Regex;
use std::path::PathBuf;

/// Context manager for tracking attached context items
pub struct ContextManager {
    /// Context items
    items: Vec<ContextItem>,
    /// Maximum total tokens
    max_tokens: usize,
    /// Current token count
    current_tokens: usize,
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new(100_000) // Default 100k token limit
    }
}

impl ContextManager {
    /// Create a new context manager with token limit
    pub fn new(max_tokens: usize) -> Self {
        Self {
            items: Vec::new(),
            max_tokens,
            current_tokens: 0,
        }
    }

    /// Add a context item
    pub fn add(&mut self, item: ContextItem) -> Result<(), ContextError> {
        // Check if adding this would exceed the limit
        if self.current_tokens + item.token_count > self.max_tokens {
            return Err(ContextError::TokenLimitExceeded {
                current: self.current_tokens,
                item: item.token_count,
                max: self.max_tokens,
            });
        }

        self.current_tokens += item.token_count;
        self.items.push(item);
        Ok(())
    }

    /// Remove a context item by ID
    pub fn remove(&mut self, id: &str) -> Option<ContextItem> {
        if let Some(pos) = self.items.iter().position(|i| i.id == id) {
            let item = self.items.remove(pos);
            self.current_tokens = self.current_tokens.saturating_sub(item.token_count);
            Some(item)
        } else {
            None
        }
    }

    /// Get all items
    pub fn items(&self) -> &[ContextItem] {
        &self.items
    }

    /// Get pinned items only
    pub fn pinned_items(&self) -> impl Iterator<Item = &ContextItem> {
        self.items.iter().filter(|i| i.pinned)
    }

    /// Clear all non-pinned items
    pub fn clear_unpinned(&mut self) {
        self.items.retain(|i| i.pinned);
        self.recalculate_tokens();
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.current_tokens = 0;
    }

    /// Get current token count
    pub fn token_count(&self) -> usize {
        self.current_tokens
    }

    /// Get remaining token budget
    pub fn remaining_tokens(&self) -> usize {
        self.max_tokens.saturating_sub(self.current_tokens)
    }

    /// Format all context for AI prompt
    pub fn format_for_prompt(&self) -> String {
        if self.items.is_empty() {
            return String::new();
        }

        let mut output = String::from("=== CONTEXT ===\n\n");

        for item in &self.items {
            output.push_str(&item.format_for_prompt());
            output.push_str("\n\n");
        }

        output.push_str("=== END CONTEXT ===\n\n");
        output
    }

    /// Recalculate token count
    fn recalculate_tokens(&mut self) {
        self.current_tokens = self.items.iter().map(|i| i.token_count).sum();
    }

    /// Find item by ID
    pub fn get(&self, id: &str) -> Option<&ContextItem> {
        self.items.iter().find(|i| i.id == id)
    }

    /// Toggle pinned status
    pub fn toggle_pin(&mut self, id: &str) -> bool {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.pinned = !item.pinned;
            item.pinned
        } else {
            false
        }
    }

    /// Parse @file references from text (e.g., @src/main.rs)
    pub fn parse_file_references(text: &str) -> Vec<PathBuf> {
        let pattern = Regex::new(r"@([^\s@]+)").unwrap();
        let mut files = Vec::new();

        for cap in pattern.captures_iter(text) {
            if let Some(path_match) = cap.get(1) {
                let path = PathBuf::from(path_match.as_str());
                // Only include if path exists (file or directory)
                if path.exists() {
                    files.push(path);
                }
            }
        }

        files
    }

    /// Add files from @file references in text
    /// Returns list of paths that were added
    pub fn add_file_references(&mut self, text: &str) -> Result<Vec<PathBuf>, ContextError> {
        let refs = Self::parse_file_references(text);
        let mut added = Vec::new();

        for path in refs {
            // Skip if already in context
            if self.items.iter().any(|i| i.path.as_ref() == Some(&path)) {
                continue;
            }

            // Read file content
            let content = std::fs::read_to_string(&path)
                .map_err(|e| ContextError::FileReadError(e.to_string()))?;

            // Add as context item
            self.add(ContextItem::file(&path, content))?;
            added.push(path);
        }

        Ok(added)
    }

    /// Get context window usage as percentage
    pub fn usage_percent(&self) -> f32 {
        if self.max_tokens == 0 {
            return 0.0;
        }
        (self.current_tokens as f32 / self.max_tokens as f32) * 100.0
    }

    /// Check if context is near limit (> 80%)
    pub fn is_near_limit(&self) -> bool {
        self.usage_percent() > 80.0
    }

    /// Check if context needs summarization (> 90%)
    pub fn needs_summarization(&self) -> bool {
        self.usage_percent() > 90.0
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::ContextItemType;
    use super::*;

    #[test]
    fn test_context_manager() {
        let mut manager = ContextManager::new(1000);

        let item1 = ContextItem::file("/a.rs", "let x = 1;"); // ~3 tokens
        let item2 = ContextItem::file("/b.rs", "let y = 2;"); // ~3 tokens

        assert!(manager.add(item1).is_ok());
        assert!(manager.add(item2).is_ok());
        assert_eq!(manager.items().len(), 2);
    }

    #[test]
    fn test_token_limit() {
        let mut manager = ContextManager::new(10); // Very small limit

        let item = ContextItem::file("/big.rs", "a".repeat(100)); // ~25 tokens
        assert!(manager.add(item).is_err());
    }

    #[test]
    fn test_pinned_items() {
        let mut manager = ContextManager::new(1000);

        let item1 = ContextItem::file("/a.rs", "code").pin();
        let item2 = ContextItem::file("/b.rs", "more code");

        manager.add(item1).unwrap();
        manager.add(item2).unwrap();

        assert_eq!(manager.pinned_items().count(), 1);

        manager.clear_unpinned();
        assert_eq!(manager.items().len(), 1);
    }
}
