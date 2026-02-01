//! Animation types for collapsible components

use gpui::*;
use gpui::prelude::*;

/// Animation duration for expand/collapse
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CollapsibleAnimation {
    #[default]
    Default,
    Fast,
    Slow,
    None,
}
