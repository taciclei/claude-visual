//! Context Panel Component
//!
//! UI component for managing attached context items (files, snippets, etc.).

mod core;
mod item_render;
mod render;
mod types;

pub use core::ContextPanel;
pub use types::ContextPanelEvent;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::context::ContextItemType;

    #[test]
    fn test_icon_for_type() {
        assert_eq!(ContextPanel::icon_for_type(&ContextItemType::File), "📄");
        assert_eq!(ContextPanel::icon_for_type(&ContextItemType::Snippet), "✂️");
        assert_eq!(ContextPanel::icon_for_type(&ContextItemType::Web), "🌐");
    }
}
