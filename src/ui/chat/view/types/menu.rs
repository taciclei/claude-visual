//! Context menu types

/// Context menu state for message actions
#[derive(Debug, Clone)]
pub struct ContextMenuState {
    /// Message index
    pub message_index: usize,
    /// Position X (in pixels from left)
    pub x: f32,
    /// Position Y (in pixels from top)
    pub y: f32,
}

impl ContextMenuState {
    pub fn new(message_index: usize, x: f32, y: f32) -> Self {
        Self { message_index, x, y }
    }
}
