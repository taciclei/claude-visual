//! Theme builder for programmatic theme creation

use crate::app::theme::{SyntaxColors, Theme, ThemeColors};

/// Theme builder for creating themes programmatically
pub struct ThemeBuilder {
    name: String,
    is_dark: bool,
    colors: Option<ThemeColors>,
    syntax: Option<SyntaxColors>,
    author: Option<String>,
    description: Option<String>,
}

impl ThemeBuilder {
    /// Create a new theme builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_dark: true,
            colors: None,
            syntax: None,
            author: None,
            description: None,
        }
    }

    /// Set dark mode
    pub fn dark(mut self) -> Self {
        self.is_dark = true;
        self
    }

    /// Set light mode
    pub fn light(mut self) -> Self {
        self.is_dark = false;
        self
    }

    /// Set theme colors
    pub fn colors(mut self, colors: ThemeColors) -> Self {
        self.colors = Some(colors);
        self
    }

    /// Set syntax colors
    pub fn syntax(mut self, syntax: SyntaxColors) -> Self {
        self.syntax = Some(syntax);
        self
    }

    /// Set author
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Build the theme
    pub fn build(self) -> Theme {
        let base = if self.is_dark {
            Theme::dark()
        } else {
            Theme::light()
        };

        Theme {
            name: self.name,
            is_dark: self.is_dark,
            variant: if self.is_dark { crate::app::theme::ThemeVariant::Dark } else { crate::app::theme::ThemeVariant::Light },
            colors: self.colors.unwrap_or(base.colors),
            syntax: self.syntax.unwrap_or(base.syntax),
            accessibility: crate::app::theme::AccessibilitySettings::default(),
        }
    }
}
