//! Lazy Loading Block Wrapper
//!
//! Provides lazy rendering for heavy UI components like code blocks,
//! only initializing content when visible in the viewport.

mod types;
mod config;
mod core;
mod render;
mod observer;

// Re-export public types
pub use types::{LazyState, LazyBlockEvent};
pub use config::LazyBlockConfig;
pub use core::LazyBlock;
pub use render::lazy_code_block;
pub use observer::VisibilityObserver;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_observer() {
        let mut observer = VisibilityObserver::new(500.0, 100.0);

        // Register blocks
        observer.register_block("block1", 0.0, 100.0);
        observer.register_block("block2", 200.0, 100.0);
        observer.register_block("block3", 800.0, 100.0);

        // Initially at top, first two should be visible
        let visible = observer.visible_blocks();
        assert!(visible.contains(&"block1"));
        assert!(visible.contains(&"block2"));
        assert!(!visible.contains(&"block3"));

        // Scroll down
        observer.set_scroll_offset(500.0);
        let visible = observer.visible_blocks();
        assert!(!visible.contains(&"block1"));
        assert!(visible.contains(&"block2")); // Within preload margin
        assert!(visible.contains(&"block3"));
    }

    #[test]
    fn test_lazy_block_config() {
        let config = LazyBlockConfig::for_code_block(50);
        assert!(config.estimated_height > 0.0);
        assert!(config.estimated_height <= 500.0);

        let config = LazyBlockConfig::for_code_block(1000);
        assert_eq!(config.estimated_height, 500.0); // Capped
    }
}
