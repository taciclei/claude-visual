//! Theme toggle type definitions

/// Theme mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    #[default]
    Dark,
    System,
}

impl ThemeMode {
    pub(crate) fn icon(&self) -> &'static str {
        match self {
            Self::Light => "☀️",
            Self::Dark => "🌙",
            Self::System => "💻",
        }
    }

    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::System => "System",
        }
    }
}

/// Theme toggle size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ThemeToggleSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ThemeToggleSize {
    pub(crate) fn height(&self) -> f32 {
        match self {
            Self::Sm => 28.0,
            Self::Md => 36.0,
            Self::Lg => 44.0,
        }
    }

    pub(crate) fn icon_size(&self) -> f32 {
        match self {
            Self::Sm => 14.0,
            Self::Md => 18.0,
            Self::Lg => 22.0,
        }
    }
}

/// Theme toggle variant
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ThemeToggleVariant {
    #[default]
    Switch,
    Button,
    Segmented,
    Dropdown,
}
