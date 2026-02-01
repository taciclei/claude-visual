//! Password strength types and shared data structures

use gpui::*;

/// Password strength level
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum PasswordStrength {
    #[default]
    None,
    VeryWeak,
    Weak,
    Fair,
    Good,
    Strong,
}

impl PasswordStrength {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::VeryWeak => "Very Weak",
            Self::Weak => "Weak",
            Self::Fair => "Fair",
            Self::Good => "Good",
            Self::Strong => "Strong",
        }
    }

    pub(crate) fn color(&self) -> gpui::Hsla {
        match self {
            Self::None => hsla(0.0, 0.0, 0.3, 1.0),
            Self::VeryWeak => hsla(0.0, 0.8, 0.5, 1.0),
            Self::Weak => hsla(0.08, 0.9, 0.5, 1.0),
            Self::Fair => hsla(0.12, 0.9, 0.5, 1.0),
            Self::Good => hsla(0.25, 0.7, 0.45, 1.0),
            Self::Strong => hsla(0.35, 0.7, 0.45, 1.0),
        }
    }

    pub(crate) fn segments(&self) -> usize {
        match self {
            Self::None => 0,
            Self::VeryWeak => 1,
            Self::Weak => 2,
            Self::Fair => 3,
            Self::Good => 4,
            Self::Strong => 5,
        }
    }

    /// Calculate strength from password
    pub fn from_password(password: &str) -> Self {
        if password.is_empty() {
            return Self::None;
        }

        let mut score = 0;

        // Length checks
        if password.len() >= 8 {
            score += 1;
        }
        if password.len() >= 12 {
            score += 1;
        }
        if password.len() >= 16 {
            score += 1;
        }

        // Character variety checks
        if password.chars().any(|c| c.is_lowercase()) {
            score += 1;
        }
        if password.chars().any(|c| c.is_uppercase()) {
            score += 1;
        }
        if password.chars().any(|c| c.is_numeric()) {
            score += 1;
        }
        if password.chars().any(|c| !c.is_alphanumeric()) {
            score += 1;
        }

        match score {
            0..=1 => Self::VeryWeak,
            2 => Self::Weak,
            3..=4 => Self::Fair,
            5..=6 => Self::Good,
            _ => Self::Strong,
        }
    }
}

/// Password strength variant
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StrengthMeterVariant {
    #[default]
    Bar,
    Segments,
    Circle,
    Text,
}

/// Password requirement item
#[derive(Clone)]
pub struct PasswordRequirement {
    pub label: SharedString,
    pub met: bool,
}

impl PasswordRequirement {
    pub fn new(label: impl Into<SharedString>, met: bool) -> Self {
        Self {
            label: label.into(),
            met,
        }
    }
}
