//! Toolbar components - Action bars and tool groups
//!
//! Provides toolbar components for grouping actions and tools.

mod floating_toolbar;
mod quick_actions;
mod toolbar;
mod toolbar_group;
mod toolbar_separator;
mod types;

pub use floating_toolbar::FloatingToolbar;
pub use quick_actions::QuickActions;
pub use toolbar::Toolbar;
pub use toolbar_group::ToolbarGroup;
pub use toolbar_separator::ToolbarSeparator;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolbar_item() {
        let item = ToolbarItem::new("bold", "𝐁")
            .label("Bold")
            .tooltip("Make text bold")
            .active(true);

        assert_eq!(item.id.as_ref(), "bold");
        assert_eq!(item.icon.as_ref(), "𝐁");
        assert!(item.active);
    }

    #[test]
    fn test_toolbar_sizes() {
        let small = Toolbar::new("s").size(ToolbarSize::Small);
        let medium = Toolbar::new("m").size(ToolbarSize::Medium);
        let large = Toolbar::new("l").size(ToolbarSize::Large);

        assert_eq!(small.size, ToolbarSize::Small);
        assert_eq!(medium.size, ToolbarSize::Medium);
        assert_eq!(large.size, ToolbarSize::Large);
    }

    #[test]
    fn test_toolbar_variants() {
        let default = Toolbar::new("d").variant(ToolbarVariant::Default);
        let floating = Toolbar::new("f").variant(ToolbarVariant::Floating);
        let attached = Toolbar::new("a").variant(ToolbarVariant::Attached);

        assert_eq!(default.variant, ToolbarVariant::Default);
        assert_eq!(floating.variant, ToolbarVariant::Floating);
        assert_eq!(attached.variant, ToolbarVariant::Attached);
    }

    #[test]
    fn test_toolbar_group() {
        let items = vec![ToolbarItem::new("a", "A"), ToolbarItem::new("b", "B")];
        let group = ToolbarGroup::new().items(items).vertical(true);

        assert!(group.vertical);
        assert_eq!(group.items.len(), 2);
    }

    #[test]
    fn test_floating_toolbar() {
        let toolbar = FloatingToolbar::new("float").position(100.0, 200.0);

        assert_eq!(toolbar.position, (100.0, 200.0));
    }
}
