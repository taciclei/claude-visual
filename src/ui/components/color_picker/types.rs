//! Color picker types

use gpui::*;

/// Events emitted by ColorPicker
#[derive(Debug, Clone)]
pub enum ColorPickerEvent {
    /// Color changed
    Changed(Hsla),
    /// Picker opened
    Opened,
    /// Picker closed
    Closed,
}
