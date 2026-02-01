//! Translation manager

use super::bundle::TranslationBundle;
use super::locales;
use crate::i18n::locale::Locale;
use std::collections::HashMap;
use std::sync::RwLock;

/// Translation manager
pub struct I18n {
    /// Current locale
    current_locale: RwLock<Locale>,
    /// Loaded bundles
    bundles: RwLock<HashMap<Locale, TranslationBundle>>,
    /// Fallback locale (usually English)
    fallback_locale: Locale,
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

impl I18n {
    /// Create a new i18n manager with default English translations
    pub fn new() -> Self {
        let mut manager = Self {
            current_locale: RwLock::new(Locale::EnUs),
            bundles: RwLock::new(HashMap::new()),
            fallback_locale: Locale::EnUs,
        };

        // Load default English translations
        manager.load_bundle(locales::english_bundle());
        // Load French translations
        manager.load_bundle(locales::french_bundle());
        // Load Spanish translations
        manager.load_bundle(locales::spanish_bundle());
        // Load German translations
        manager.load_bundle(locales::german_bundle());
        // Load Japanese translations
        manager.load_bundle(locales::japanese_bundle());
        // Load Portuguese translations
        manager.load_bundle(locales::portuguese_bundle());
        // Load Chinese translations
        manager.load_bundle(locales::chinese_bundle());

        manager
    }

    /// Load a translation bundle
    pub fn load_bundle(&mut self, bundle: TranslationBundle) {
        let mut bundles = self.bundles.write().unwrap();
        bundles.insert(bundle.locale(), bundle);
    }

    /// Set the current locale
    pub fn set_locale(&self, locale: Locale) {
        let mut current = self.current_locale.write().unwrap();
        *current = locale;
    }

    /// Get the current locale
    pub fn locale(&self) -> Locale {
        *self.current_locale.read().unwrap()
    }

    /// Get a translated message
    pub fn t(&self, id: &str) -> String {
        let locale = self.locale();
        let bundles = self.bundles.read().unwrap();

        // Try current locale first
        if let Some(bundle) = bundles.get(&locale) {
            if let Some(msg) = bundle.get(id) {
                return msg.to_string();
            }
        }

        // Fall back to English
        if locale != self.fallback_locale {
            if let Some(bundle) = bundles.get(&self.fallback_locale) {
                if let Some(msg) = bundle.get(id) {
                    return msg.to_string();
                }
            }
        }

        // Return the message ID if not found
        id.to_string()
    }

    /// Get a translated message with arguments
    pub fn tf(&self, id: &str, args: &HashMap<String, String>) -> String {
        let locale = self.locale();
        let bundles = self.bundles.read().unwrap();

        // Try current locale first
        if let Some(bundle) = bundles.get(&locale) {
            if let Some(msg) = bundle.format(id, args) {
                return msg;
            }
        }

        // Fall back to English
        if locale != self.fallback_locale {
            if let Some(bundle) = bundles.get(&self.fallback_locale) {
                if let Some(msg) = bundle.format(id, args) {
                    return msg;
                }
            }
        }

        // Return the message ID if not found
        id.to_string()
    }

    /// Check if a locale has translations
    pub fn has_locale(&self, locale: Locale) -> bool {
        let bundles = self.bundles.read().unwrap();
        bundles.contains_key(&locale)
    }

    /// Get available locales
    pub fn available_locales(&self) -> Vec<Locale> {
        let bundles = self.bundles.read().unwrap();
        bundles.keys().copied().collect()
    }
}
