//! Shared types for accordion components

/// Accordion expansion modes
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AccordionMode {
    /// Only one item can be expanded at a time (default)
    #[default]
    Single,
    /// Multiple items can be expanded
    Multiple,
}

/// Accordion style variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AccordionStyle {
    /// Default with borders
    #[default]
    Default,
    /// Separated cards
    Separated,
    /// Minimal without borders
    Minimal,
    /// Flush with no outer borders
    Flush,
}

/// Events emitted by Accordion
#[derive(Debug, Clone)]
pub enum AccordionEvent {
    /// Item expanded
    Expanded(usize),
    /// Item collapsed
    Collapsed(usize),
}

/// Accordion item data
#[derive(Debug, Clone)]
pub struct AccordionItem {
    /// Item title/header
    pub title: String,
    /// Optional subtitle
    pub subtitle: Option<String>,
    /// Optional icon
    pub icon: Option<String>,
    /// Whether item is disabled
    pub disabled: bool,
}

impl AccordionItem {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            icon: None,
            disabled: false,
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Simple accordion item data
#[derive(Clone)]
pub struct SimpleAccordionItem {
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) icon: Option<String>,
}

impl SimpleAccordionItem {
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            icon: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// FAQ-style accordion item
#[derive(Clone)]
pub struct FaqItem {
    pub(crate) question: String,
    pub(crate) answer: String,
}

impl FaqItem {
    pub fn new(question: impl Into<String>, answer: impl Into<String>) -> Self {
        Self {
            question: question.into(),
            answer: answer.into(),
        }
    }
}
