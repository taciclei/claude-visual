//! Translation management
//!
//! Provides a simple translation system inspired by Fluent.

use std::collections::HashMap;

pub mod bundle;
pub mod locales;
pub mod manager;

pub use bundle::TranslationBundle;
pub use manager::I18n;

/// Global i18n instance
static I18N: std::sync::OnceLock<I18n> = std::sync::OnceLock::new();

/// Get the global i18n instance
pub fn i18n() -> &'static I18n {
    I18N.get_or_init(I18n::new)
}

/// Convenience function to get a translation
pub fn t(id: &str) -> String {
    i18n().t(id)
}

/// Convenience function to get a translation with arguments
pub fn tf(id: &str, args: &HashMap<String, String>) -> String {
    i18n().tf(id, args)
}

/// Convenience macro for translation with arguments
#[macro_export]
macro_rules! t {
    ($id:expr) => {
        $crate::i18n::t($id)
    };
    ($id:expr, $($key:ident = $value:expr),* $(,)?) => {
        {
            let mut args = std::collections::HashMap::new();
            $(
                args.insert(stringify!($key).to_string(), $value.to_string());
            )*
            $crate::i18n::tf($id, &args)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::locale::Locale;

    #[test]
    fn test_english_translations() {
        let i18n = I18n::new();
        assert_eq!(i18n.t("common.ok"), "OK");
        assert_eq!(i18n.t("common.cancel"), "Cancel");
    }

    #[test]
    fn test_french_translations() {
        let i18n = I18n::new();
        i18n.set_locale(Locale::FrFr);
        assert_eq!(i18n.t("common.ok"), "OK");
        assert_eq!(i18n.t("common.cancel"), "Annuler");
    }

    #[test]
    fn test_fallback() {
        let i18n = I18n::new();
        // Set to a locale without translations loaded
        i18n.set_locale(Locale::DeDe);
        // Should fall back to English
        assert_eq!(i18n.t("common.ok"), "OK");
    }

    #[test]
    fn test_format_args() {
        let i18n = I18n::new();
        let mut args = HashMap::new();
        args.insert("count".to_string(), "42".to_string());

        let result = i18n.tf("code.lines", &args);
        assert_eq!(result, "42 lines");
    }

    #[test]
    fn test_missing_key() {
        let i18n = I18n::new();
        // Missing keys should return the key itself
        assert_eq!(i18n.t("nonexistent.key"), "nonexistent.key");
    }

    #[test]
    fn test_spanish_translations() {
        let i18n = I18n::new();
        i18n.set_locale(Locale::EsEs);
        assert_eq!(i18n.t("common.ok"), "Aceptar");
        assert_eq!(i18n.t("common.cancel"), "Cancelar");
        assert_eq!(i18n.t("chat.send"), "Enviar");
        assert_eq!(i18n.t("settings.title"), "Configuración");
    }

    #[test]
    fn test_german_translations() {
        let i18n = I18n::new();
        i18n.set_locale(Locale::DeDe);
        assert_eq!(i18n.t("common.ok"), "OK");
        assert_eq!(i18n.t("common.cancel"), "Abbrechen");
        assert_eq!(i18n.t("chat.send"), "Senden");
        assert_eq!(i18n.t("settings.title"), "Einstellungen");
    }

    #[test]
    fn test_japanese_translations() {
        let i18n = I18n::new();
        i18n.set_locale(Locale::JaJp);
        assert_eq!(i18n.t("common.ok"), "OK");
        assert_eq!(i18n.t("common.cancel"), "キャンセル");
        assert_eq!(i18n.t("chat.send"), "送信");
        assert_eq!(i18n.t("settings.title"), "設定");
    }

    #[test]
    fn test_portuguese_translations() {
        let i18n = I18n::new();
        i18n.set_locale(Locale::PtBr);
        assert_eq!(i18n.t("common.ok"), "OK");
        assert_eq!(i18n.t("common.cancel"), "Cancelar");
        assert_eq!(i18n.t("chat.send"), "Enviar");
        assert_eq!(i18n.t("settings.title"), "Configurações");
    }

    #[test]
    fn test_chinese_translations() {
        let i18n = I18n::new();
        i18n.set_locale(Locale::ZhCn);
        assert_eq!(i18n.t("common.ok"), "确定");
        assert_eq!(i18n.t("common.cancel"), "取消");
        assert_eq!(i18n.t("chat.send"), "发送");
        assert_eq!(i18n.t("settings.title"), "设置");
    }

    #[test]
    fn test_available_locales() {
        let i18n = I18n::new();
        let locales = i18n.available_locales();
        assert!(locales.contains(&Locale::EnUs));
        assert!(locales.contains(&Locale::FrFr));
        assert!(locales.contains(&Locale::EsEs));
        assert!(locales.contains(&Locale::DeDe));
        assert!(locales.contains(&Locale::JaJp));
        assert!(locales.contains(&Locale::PtBr));
        assert!(locales.contains(&Locale::ZhCn));
    }

    #[test]
    fn test_all_locales_have_translations() {
        let i18n = I18n::new();
        // Test that all locales have the basic keys
        for locale in Locale::all() {
            if i18n.has_locale(*locale) {
                i18n.set_locale(*locale);
                // Should have common keys
                assert!(!i18n.t("common.ok").is_empty());
                assert!(!i18n.t("common.cancel").is_empty());
                assert!(!i18n.t("settings.title").is_empty());
            }
        }
    }
}
