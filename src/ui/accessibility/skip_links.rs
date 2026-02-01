//! Skip links for keyboard navigation
//!
//! Allows keyboard users to quickly navigate to main content areas.

use serde::{Deserialize, Serialize};

/// A skip link target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkipLinkTarget {
    /// Unique identifier for the target
    pub id: String,
    /// Display label for the link
    pub label: String,
    /// Keyboard shortcut (optional)
    pub shortcut: Option<String>,
    /// Priority (lower = earlier in list)
    pub priority: u32,
}

impl SkipLinkTarget {
    /// Create a new skip link target
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shortcut: None,
            priority: 100,
        }
    }

    /// Set the keyboard shortcut
    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set the priority
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
}

/// Default skip link targets
pub struct SkipLinks;

impl SkipLinks {
    /// Skip to main content
    pub fn main_content() -> SkipLinkTarget {
        SkipLinkTarget::new("main-content", "Skip to main content")
            .with_shortcut("Alt+1")
            .with_priority(0)
    }

    /// Skip to chat input
    pub fn chat_input() -> SkipLinkTarget {
        SkipLinkTarget::new("chat-input", "Skip to chat input")
            .with_shortcut("Alt+2")
            .with_priority(1)
    }

    /// Skip to navigation
    pub fn navigation() -> SkipLinkTarget {
        SkipLinkTarget::new("navigation", "Skip to navigation")
            .with_shortcut("Alt+3")
            .with_priority(2)
    }

    /// Skip to sidebar
    pub fn sidebar() -> SkipLinkTarget {
        SkipLinkTarget::new("sidebar", "Skip to sidebar")
            .with_shortcut("Alt+4")
            .with_priority(3)
    }

    /// Get all default skip links
    pub fn defaults() -> Vec<SkipLinkTarget> {
        let mut links = vec![
            Self::main_content(),
            Self::chat_input(),
            Self::navigation(),
            Self::sidebar(),
        ];
        links.sort_by_key(|l| l.priority);
        links
    }
}

/// Skip link manager
#[derive(Debug, Default)]
pub struct SkipLinkManager {
    /// Registered skip link targets
    targets: Vec<SkipLinkTarget>,
    /// Whether skip links are visible
    visible: bool,
}

impl SkipLinkManager {
    /// Create a new skip link manager with defaults
    pub fn new() -> Self {
        Self {
            targets: SkipLinks::defaults(),
            visible: false,
        }
    }

    /// Create an empty skip link manager
    pub fn empty() -> Self {
        Self::default()
    }

    /// Register a skip link target
    pub fn register(&mut self, target: SkipLinkTarget) {
        // Remove existing target with same ID
        self.targets.retain(|t| t.id != target.id);
        self.targets.push(target);
        self.targets.sort_by_key(|t| t.priority);
    }

    /// Unregister a skip link target
    pub fn unregister(&mut self, id: &str) {
        self.targets.retain(|t| t.id != id);
    }

    /// Get all registered targets
    pub fn targets(&self) -> &[SkipLinkTarget] {
        &self.targets
    }

    /// Show skip links (on Tab key before any focus)
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide skip links
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Check if skip links are visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get target by shortcut
    pub fn target_by_shortcut(&self, shortcut: &str) -> Option<&SkipLinkTarget> {
        self.targets
            .iter()
            .find(|t| t.shortcut.as_ref().map(|s| s == shortcut).unwrap_or(false))
    }

    /// Get target by ID
    pub fn target_by_id(&self, id: &str) -> Option<&SkipLinkTarget> {
        self.targets.iter().find(|t| t.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_links() {
        let manager = SkipLinkManager::new();
        let targets = manager.targets();

        // Should have defaults
        assert!(!targets.is_empty());

        // Should be sorted by priority
        for i in 1..targets.len() {
            assert!(targets[i - 1].priority <= targets[i].priority);
        }
    }

    #[test]
    fn test_shortcut_lookup() {
        let manager = SkipLinkManager::new();

        let target = manager.target_by_shortcut("Alt+1");
        assert!(target.is_some());
        assert_eq!(target.unwrap().id, "main-content");
    }

    #[test]
    fn test_custom_target() {
        let mut manager = SkipLinkManager::empty();

        manager.register(
            SkipLinkTarget::new("custom", "Custom Target")
                .with_shortcut("Alt+0")
                .with_priority(0),
        );

        assert_eq!(manager.targets().len(), 1);
        assert_eq!(manager.targets()[0].id, "custom");
    }
}
