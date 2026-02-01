//! Drawer component for slide-out panels

mod types;
mod drawer_panel;
mod navigation_drawer;
mod settings_drawer;

pub use types::*;
pub use drawer_panel::*;
pub use navigation_drawer::*;
pub use settings_drawer::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawer_panel() {
        let drawer = DrawerPanel::new()
            .position(DrawerPosition::Right)
            .size(DrawerSize::Large)
            .title("Panel Title");

        assert_eq!(drawer.position, DrawerPosition::Right);
        assert!(matches!(drawer.size, DrawerSize::Large));
    }

    #[test]
    fn test_navigation_drawer() {
        let nav = NavigationDrawer::new("Menu")
            .item_with_icon("Home", "🏠")
            .item_with_icon("Settings", "⚙️")
            .selected(0);

        assert_eq!(nav.items.len(), 2);
        assert_eq!(nav.selected, Some(0));
    }

    #[test]
    fn test_settings_drawer() {
        let settings = SettingsDrawer::new("Settings")
            .section("General", vec![
                SettingsItem {
                    label: "Dark Mode".to_string(),
                    description: Some("Enable dark theme".to_string()),
                    item_type: SettingsItemType::Toggle(true),
                },
            ]);

        assert_eq!(settings.sections.len(), 1);
        assert_eq!(settings.sections[0].items.len(), 1);
    }
}
