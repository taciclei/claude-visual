//! Language settings

use serde::{Deserialize, Serialize};

use crate::i18n::Locale;

/// Language setting - either auto-detect or a specific locale
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "value")]
pub enum LanguageSetting {
    /// Auto-detect from system locale
    Auto,
    /// Specific locale
    Specific(String),
}

impl Default for LanguageSetting {
    fn default() -> Self {
        Self::Auto
    }
}

impl LanguageSetting {
    /// Get the effective locale
    pub fn effective_locale(&self) -> Locale {
        match self {
            Self::Auto => crate::i18n::detect_system_locale(),
            Self::Specific(tag) => Locale::from_tag(tag).unwrap_or_default(),
        }
    }

    /// Check if this is auto-detect mode
    pub fn is_auto(&self) -> bool {
        matches!(self, Self::Auto)
    }

    /// Get display name for the setting
    pub fn display_name(&self) -> String {
        match self {
            Self::Auto => {
                let detected = crate::i18n::detect_system_locale();
                format!("Auto ({})", detected.native_name())
            }
            Self::Specific(tag) => {
                if let Some(locale) = Locale::from_tag(tag) {
                    locale.display_name()
                } else {
                    tag.clone()
                }
            }
        }
    }
}
