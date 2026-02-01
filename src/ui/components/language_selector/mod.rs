//! Language selector components
//!
//! Provides language/locale switching functionality.

mod language_dropdown;
mod language_selector;
mod locale_display;
mod translation_status;
mod types;

pub use language_dropdown::*;
pub use language_selector::*;
pub use locale_display::*;
pub use translation_status::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let en = Language::english();
        let ar = Language::arabic();

        assert_eq!(en.code.as_ref(), "en");
        assert!(!en.rtl);
        assert!(ar.rtl);
    }

    #[test]
    fn test_language_selector_sizes() {
        let sm = LanguageSelectorSize::Sm;
        let lg = LanguageSelectorSize::Lg;

        assert!(sm.height() < lg.height());
    }

    #[test]
    fn test_language_selector() {
        let selector = LanguageSelector::new("ls")
            .selected("fr")
            .variant(LanguageSelectorVariant::Dropdown)
            .show_flag(true);

        assert_eq!(
            selector.selected_code.as_ref().map(|s| s.as_ref()),
            Some("fr")
        );
        assert!(selector.show_flag);
    }

    #[test]
    fn test_language_dropdown() {
        let dropdown = LanguageDropdown::new("ld")
            .selected("de")
            .show_native_name(true);

        assert_eq!(
            dropdown.selected_code.as_ref().map(|s| s.as_ref()),
            Some("de")
        );
        assert!(dropdown.show_native_name);
    }

    #[test]
    fn test_locale_display() {
        let locale = LocaleDisplay::new("ld", "en").region("US");

        assert_eq!(locale.language_code.as_ref(), "en");
        assert_eq!(locale.region_code.as_ref().map(|s| s.as_ref()), Some("US"));
    }

    #[test]
    fn test_translation_status() {
        let status = TranslationStatus::new("ts", "French")
            .total(100)
            .translated(85);

        assert_eq!(status.total_strings, 100);
        assert_eq!(status.translated_strings, 85);
    }
}
