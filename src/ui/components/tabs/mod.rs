//! Tabs component for tabbed interfaces

mod types;
mod tabs_component;
mod render;
mod tab_bar;

pub use types::*;
pub use tabs_component::Tabs;
pub use tab_bar::TabBar;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tabs_size() {
        assert_eq!(TabsSize::Small.height(), 32.0);
        assert_eq!(TabsSize::Medium.height(), 40.0);
        assert_eq!(TabsSize::Large.height(), 48.0);
    }

    #[test]
    fn test_tab_item_builder() {
        let tab = TabItem::new("test", "Test Tab")
            .with_icon("📁")
            .with_badge(5)
            .closable()
            .disabled();

        assert_eq!(tab.id, "test");
        assert_eq!(tab.label, "Test Tab");
        assert_eq!(tab.icon, Some("📁".to_string()));
        assert_eq!(tab.badge, Some(5));
        assert!(tab.closable);
        assert!(tab.disabled);
    }
}
