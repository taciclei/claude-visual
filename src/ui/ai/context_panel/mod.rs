//! Context Panel Component
//!
//! UI component for managing attached context items (files, snippets, etc.).

mod types;
mod core;
mod render;
mod item_render;

pub use types::ContextPanelEvent;
pub use core::ContextPanel;

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
