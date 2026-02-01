//! Shared type definitions for list components

/// List size
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ListSize {
    /// Compact
    Compact,
    /// Medium (default)
    #[default]
    Medium,
    /// Comfortable
    Comfortable,
}

impl ListSize {
    pub(crate) fn item_padding(&self) -> (f32, f32) {
        match self {
            ListSize::Compact => (8.0, 4.0),
            ListSize::Medium => (12.0, 8.0),
            ListSize::Comfortable => (16.0, 12.0),
        }
    }

    pub(crate) fn gap(&self) -> f32 {
        match self {
            ListSize::Compact => 2.0,
            ListSize::Medium => 4.0,
            ListSize::Comfortable => 8.0,
        }
    }
}

/// List style
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ListStyle {
    /// Plain (default)
    #[default]
    Plain,
    /// With separators between items
    Separated,
    /// Card-style items
    Card,
    /// Striped alternating background
    Striped,
}

/// List item
#[derive(Clone)]
pub struct ListItem {
    /// Primary content
    pub(crate) primary: String,
    /// Secondary content
    pub(crate) secondary: Option<String>,
    /// Leading element (icon, avatar, etc)
    pub(crate) leading: Option<String>,
    /// Trailing element
    pub(crate) trailing: Option<String>,
    /// Whether selected
    pub(crate) selected: bool,
    /// Whether disabled
    pub(crate) disabled: bool,
    /// Clickable
    pub(crate) clickable: bool,
}

impl ListItem {
    pub fn new(primary: impl Into<String>) -> Self {
        Self {
            primary: primary.into(),
            secondary: None,
            leading: None,
            trailing: None,
            selected: false,
            disabled: false,
            clickable: true,
        }
    }

    pub fn secondary(mut self, text: impl Into<String>) -> Self {
        self.secondary = Some(text.into());
        self
    }

    pub fn leading(mut self, element: impl Into<String>) -> Self {
        self.leading = Some(element.into());
        self
    }

    pub fn trailing(mut self, element: impl Into<String>) -> Self {
        self.trailing = Some(element.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self.clickable = false;
        self
    }

    pub fn not_clickable(mut self) -> Self {
        self.clickable = false;
        self
    }
}

/// Action item
#[derive(Clone)]
pub struct ActionItem {
    pub label: String,
    pub icon: Option<String>,
    pub shortcut: Option<String>,
    pub description: Option<String>,
    pub disabled: bool,
    pub danger: bool,
}

impl ActionItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            shortcut: None,
            description: None,
            disabled: false,
            danger: false,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }
}

/// Description list layout
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DescriptionLayout {
    /// Horizontal (default)
    #[default]
    Horizontal,
    /// Vertical
    Vertical,
    /// Grid
    Grid,
}
