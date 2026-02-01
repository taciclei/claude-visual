//! Shared types for language selector components

use gpui::*;

/// Language item
#[derive(Clone)]
pub struct Language {
    pub code: SharedString,
    pub name: SharedString,
    pub native_name: SharedString,
    pub flag: Option<SharedString>,
    pub rtl: bool,
}

impl Language {
    pub fn new(
        code: impl Into<SharedString>,
        name: impl Into<SharedString>,
        native_name: impl Into<SharedString>,
    ) -> Self {
        Self {
            code: code.into(),
            name: name.into(),
            native_name: native_name.into(),
            flag: None,
            rtl: false,
        }
    }

    pub fn flag(mut self, flag: impl Into<SharedString>) -> Self {
        self.flag = Some(flag.into());
        self
    }

    pub fn rtl(mut self, rtl: bool) -> Self {
        self.rtl = rtl;
        self
    }

    /// Common languages
    pub fn english() -> Self {
        Self::new("en", "English", "English").flag("🇺🇸")
    }

    pub fn french() -> Self {
        Self::new("fr", "French", "Français").flag("🇫🇷")
    }

    pub fn spanish() -> Self {
        Self::new("es", "Spanish", "Español").flag("🇪🇸")
    }

    pub fn german() -> Self {
        Self::new("de", "German", "Deutsch").flag("🇩🇪")
    }

    pub fn portuguese() -> Self {
        Self::new("pt", "Portuguese", "Português").flag("🇵🇹")
    }

    pub fn chinese() -> Self {
        Self::new("zh", "Chinese", "中文").flag("🇨🇳")
    }

    pub fn japanese() -> Self {
        Self::new("ja", "Japanese", "日本語").flag("🇯🇵")
    }

    pub fn korean() -> Self {
        Self::new("ko", "Korean", "한국어").flag("🇰🇷")
    }

    pub fn arabic() -> Self {
        Self::new("ar", "Arabic", "العربية").flag("🇸🇦").rtl(true)
    }

    pub fn russian() -> Self {
        Self::new("ru", "Russian", "Русский").flag("🇷🇺")
    }
}

/// Language selector size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LanguageSelectorSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl LanguageSelectorSize {
    pub(crate) fn height(&self) -> f32 {
        match self {
            Self::Sm => 32.0,
            Self::Md => 40.0,
            Self::Lg => 48.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            Self::Sm => 12.0,
            Self::Md => 14.0,
            Self::Lg => 16.0,
        }
    }
}

/// Language selector variant
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LanguageSelectorVariant {
    #[default]
    Dropdown,
    Button,
    Minimal,
    Flags,
}
