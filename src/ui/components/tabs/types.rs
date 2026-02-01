//! Tabs types and enums

/// Tab style variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TabsStyle {
    /// Default underline style
    #[default]
    Underline,
    /// Pill/segment style
    Pill,
    /// Boxed tabs
    Boxed,
    /// Minimal (just text, no background)
    Minimal,
}

/// Tab size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TabsSize {
    /// Small
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large
    Large,
}

impl TabsSize {
    pub(crate) fn height(&self) -> f32 {
        match self {
            TabsSize::Small => 32.0,
            TabsSize::Medium => 40.0,
            TabsSize::Large => 48.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            TabsSize::Small => 12.0,
            TabsSize::Medium => 14.0,
            TabsSize::Large => 16.0,
        }
    }

    pub(crate) fn padding_x(&self) -> f32 {
        match self {
            TabsSize::Small => 12.0,
            TabsSize::Medium => 16.0,
            TabsSize::Large => 20.0,
        }
    }
}

/// A tab item
#[derive(Clone, Debug)]
pub struct TabItem {
    /// Unique identifier
    pub id: String,
    /// Display label
    pub label: String,
    /// Optional icon
    pub icon: Option<String>,
    /// Optional badge count
    pub badge: Option<u32>,
    /// Whether tab is disabled
    pub disabled: bool,
    /// Whether tab is closable
    pub closable: bool,
}

impl TabItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            badge: None,
            disabled: false,
            closable: false,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_badge(mut self, count: u32) -> Self {
        self.badge = Some(count);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn closable(mut self) -> Self {
        self.closable = true;
        self
    }
}

/// Events emitted by Tabs
#[derive(Debug, Clone)]
pub enum TabsEvent {
    /// Tab selection changed
    Changed(String),
    /// Tab close button clicked
    CloseRequested(String),
}
