//! Locale and language management

use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported locales
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Locale {
    /// English (United States) - Default
    #[default]
    EnUs,
    /// French (France)
    FrFr,
    /// Spanish (Spain)
    EsEs,
    /// German (Germany)
    DeDe,
    /// Japanese (Japan)
    JaJp,
    /// Portuguese (Brazil)
    PtBr,
    /// Chinese (Simplified)
    ZhCn,
    /// Korean (South Korea)
    KoKr,
}

impl Locale {
    /// Get all supported locales
    pub fn all() -> &'static [Locale] {
        &[
            Locale::EnUs,
            Locale::FrFr,
            Locale::EsEs,
            Locale::DeDe,
            Locale::JaJp,
            Locale::PtBr,
            Locale::ZhCn,
            Locale::KoKr,
        ]
    }

    /// Get the BCP 47 language tag
    pub fn language_tag(&self) -> &'static str {
        match self {
            Locale::EnUs => "en-US",
            Locale::FrFr => "fr-FR",
            Locale::EsEs => "es-ES",
            Locale::DeDe => "de-DE",
            Locale::JaJp => "ja-JP",
            Locale::PtBr => "pt-BR",
            Locale::ZhCn => "zh-CN",
            Locale::KoKr => "ko-KR",
        }
    }

    /// Get the short language code
    pub fn language_code(&self) -> &'static str {
        match self {
            Locale::EnUs => "en",
            Locale::FrFr => "fr",
            Locale::EsEs => "es",
            Locale::DeDe => "de",
            Locale::JaJp => "ja",
            Locale::PtBr => "pt",
            Locale::ZhCn => "zh",
            Locale::KoKr => "ko",
        }
    }

    /// Get the native language name
    pub fn native_name(&self) -> &'static str {
        match self {
            Locale::EnUs => "English",
            Locale::FrFr => "Français",
            Locale::EsEs => "Español",
            Locale::DeDe => "Deutsch",
            Locale::JaJp => "日本語",
            Locale::PtBr => "Português",
            Locale::ZhCn => "简体中文",
            Locale::KoKr => "한국어",
        }
    }

    /// Get the English name
    pub fn english_name(&self) -> &'static str {
        match self {
            Locale::EnUs => "English (US)",
            Locale::FrFr => "French",
            Locale::EsEs => "Spanish",
            Locale::DeDe => "German",
            Locale::JaJp => "Japanese",
            Locale::PtBr => "Portuguese (Brazil)",
            Locale::ZhCn => "Chinese (Simplified)",
            Locale::KoKr => "Korean",
        }
    }

    /// Get display name (native + english)
    pub fn display_name(&self) -> String {
        if self.native_name() == self.english_name() {
            self.native_name().to_string()
        } else {
            format!("{} ({})", self.native_name(), self.english_name())
        }
    }

    /// Parse from language tag
    pub fn from_tag(tag: &str) -> Option<Locale> {
        let normalized = tag.to_lowercase().replace('_', "-");

        // Try exact match first
        for locale in Self::all() {
            if locale.language_tag().to_lowercase() == normalized {
                return Some(*locale);
            }
        }

        // Try language code only
        let lang = normalized.split('-').next()?;
        for locale in Self::all() {
            if locale.language_code() == lang {
                return Some(*locale);
            }
        }

        None
    }

    /// Get text direction
    pub fn direction(&self) -> TextDirection {
        // All currently supported locales are LTR
        // Add RTL support for Arabic, Hebrew when added
        TextDirection::Ltr
    }

    /// Get the flag emoji for the locale
    pub fn flag_emoji(&self) -> &'static str {
        match self {
            Locale::EnUs => "🇺🇸",
            Locale::FrFr => "🇫🇷",
            Locale::EsEs => "🇪🇸",
            Locale::DeDe => "🇩🇪",
            Locale::JaJp => "🇯🇵",
            Locale::PtBr => "🇧🇷",
            Locale::ZhCn => "🇨🇳",
            Locale::KoKr => "🇰🇷",
        }
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.language_tag())
    }
}

/// Text direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirection {
    /// Left to right
    Ltr,
    /// Right to left
    Rtl,
}

impl Default for TextDirection {
    fn default() -> Self {
        Self::Ltr
    }
}

/// Detect system locale
pub fn detect_system_locale() -> Locale {
    // Try environment variables
    if let Ok(lang) = std::env::var("LANG") {
        if let Some(locale) = Locale::from_tag(&lang) {
            return locale;
        }
    }

    if let Ok(lang) = std::env::var("LC_ALL") {
        if let Some(locale) = Locale::from_tag(&lang) {
            return locale;
        }
    }

    if let Ok(lang) = std::env::var("LC_MESSAGES") {
        if let Some(locale) = Locale::from_tag(&lang) {
            return locale;
        }
    }

    // macOS specific
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("defaults")
            .args(["read", "-g", "AppleLocale"])
            .output()
        {
            if output.status.success() {
                let locale_str = String::from_utf8_lossy(&output.stdout);
                if let Some(locale) = Locale::from_tag(locale_str.trim()) {
                    return locale;
                }
            }
        }
    }

    // Default to English
    Locale::EnUs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_parsing() {
        assert_eq!(Locale::from_tag("en-US"), Some(Locale::EnUs));
        assert_eq!(Locale::from_tag("fr-FR"), Some(Locale::FrFr));
        assert_eq!(Locale::from_tag("fr_FR"), Some(Locale::FrFr));
        assert_eq!(Locale::from_tag("FR"), Some(Locale::FrFr));
        assert_eq!(Locale::from_tag("ja"), Some(Locale::JaJp));
    }

    #[test]
    fn test_locale_properties() {
        let fr = Locale::FrFr;
        assert_eq!(fr.language_tag(), "fr-FR");
        assert_eq!(fr.language_code(), "fr");
        assert_eq!(fr.native_name(), "Français");
        assert_eq!(fr.english_name(), "French");
    }
}
