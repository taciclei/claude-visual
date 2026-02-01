//! Type definitions for scroll area components

/// Scrollbar visibility options
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ScrollbarVisibility {
    #[default]
    Auto,
    Always,
    Hover,
    Never,
}

/// Scrollbar size options
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ScrollbarSize {
    Thin,
    #[default]
    Default,
    Thick,
}

impl ScrollbarSize {
    pub fn width(&self) -> f32 {
        match self {
            ScrollbarSize::Thin => 4.0,
            ScrollbarSize::Default => 8.0,
            ScrollbarSize::Thick => 12.0,
        }
    }
}

/// Scroll direction
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ScrollDirection {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

/// Scroll button position
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ScrollButtonPosition {
    #[default]
    Right,
    Left,
    Center,
}
