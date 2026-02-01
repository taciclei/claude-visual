//! Shared types for textarea components

/// Textarea resize modes
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TextareaResize {
    /// No resizing
    None,
    /// Vertical resize only (default)
    #[default]
    Vertical,
    /// Horizontal resize only
    Horizontal,
    /// Both directions
    Both,
    /// Auto-expand to fit content
    Auto,
}

/// Events emitted by Textarea
#[derive(Debug, Clone)]
pub enum TextareaEvent {
    /// Text content changed
    Changed(String),
    /// Focus gained
    Focus,
    /// Focus lost
    Blur,
    /// Submit (Cmd+Enter or Ctrl+Enter)
    Submit(String),
}
