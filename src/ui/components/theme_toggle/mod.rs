//! Theme toggle components
//!
//! Provides components for switching between light/dark themes.

mod types;
mod theme_toggle;
mod theme_button;
mod theme_preview;
mod appearance_settings;

pub use types::*;
pub use theme_toggle::ThemeToggle;
pub use theme_button::ThemeButton;
pub use theme_preview::ThemePreview;
pub use appearance_settings::AppearanceSettings;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_modes() {
        assert_eq!(ThemeMode::Light.label(), "Light");
        assert_eq!(ThemeMode::Dark.label(), "Dark");
        assert_eq!(ThemeMode::System.label(), "System");
    }

    #[test]
    fn test_theme_toggle_sizes() {
        let sm = ThemeToggleSize::Sm;
        let lg = ThemeToggleSize::Lg;

        assert!(sm.height() < lg.height());
        assert!(sm.icon_size() < lg.icon_size());
    }

    #[test]
    fn test_theme_toggle_variants() {
        let switch = ThemeToggle::new("s").variant(ThemeToggleVariant::Switch);
        let button = ThemeToggle::new("b").variant(ThemeToggleVariant::Button);
        let segmented = ThemeToggle::new("sg").variant(ThemeToggleVariant::Segmented);

        assert_eq!(switch.variant, ThemeToggleVariant::Switch);
        assert_eq!(button.variant, ThemeToggleVariant::Button);
        assert_eq!(segmented.variant, ThemeToggleVariant::Segmented);
    }

    #[test]
    fn test_theme_toggle() {
        let toggle = ThemeToggle::new("tt")
            .mode(ThemeMode::Dark)
            .show_label(true)
            .include_system(true);

        assert_eq!(toggle.mode, ThemeMode::Dark);
        assert!(toggle.show_label);
        assert!(toggle.include_system);
    }

    #[test]
    fn test_theme_preview() {
        let preview = ThemePreview::new("tp", ThemeMode::Light)
            .selected(true);

        assert!(preview.is_selected);
    }

    #[test]
    fn test_appearance_settings() {
        let settings = AppearanceSettings::new("as")
            .current_mode(ThemeMode::Dark)
            .selected_accent(2);

        assert_eq!(settings.current_mode, ThemeMode::Dark);
        assert_eq!(settings.selected_accent, 2);
    }
}
