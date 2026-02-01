//! Context Panel Types

use gpui::*;

pub(crate) struct SimpleColors {
    pub(crate) surface: Hsla,
    pub(crate) surface_hover: Hsla,
    pub(crate) border: Hsla,
    pub(crate) text: Hsla,
    pub(crate) text_muted: Hsla,
    pub(crate) accent: Hsla,
    pub(crate) error: Hsla,
    pub(crate) success: Hsla,
    pub(crate) warning: Hsla,
    pub(crate) background: Hsla,
}

pub(crate) fn default_colors() -> SimpleColors {
    SimpleColors {
        surface: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
        surface_hover: hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
        border: hsla(220.0 / 360.0, 0.13, 0.20, 1.0),
        text: hsla(0.0, 0.0, 0.93, 1.0),
        text_muted: hsla(0.0, 0.0, 0.60, 1.0),
        accent: hsla(210.0 / 360.0, 0.80, 0.55, 1.0),
        error: hsla(0.0, 0.84, 0.60, 1.0),
        success: hsla(142.0 / 360.0, 0.71, 0.45, 1.0),
        warning: hsla(38.0 / 360.0, 0.92, 0.50, 1.0),
        background: hsla(220.0 / 360.0, 0.13, 0.09, 1.0),
    }
}

/// Event emitted by context panel
#[derive(Clone, Debug)]
pub enum ContextPanelEvent {
    /// Add file to context
    AddFile(std::path::PathBuf),
    /// Remove item from context
    RemoveItem(String),
    /// Toggle pin on item
    TogglePin(String),
    /// Clear all context
    ClearAll,
    /// Attach file requested (opens file picker)
    AttachFileRequested,
    /// Attach snippet requested
    AttachSnippetRequested,
}
