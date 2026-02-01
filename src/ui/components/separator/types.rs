//! Shared types for separator components

/// Separator orientation
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Separator style variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SeparatorStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
    Gradient,
}

/// Separator thickness
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SeparatorThickness {
    Thin,
    #[default]
    Default,
    Thick,
}

impl SeparatorThickness {
    pub(crate) fn pixels(&self) -> f32 {
        match self {
            SeparatorThickness::Thin => 1.0,
            SeparatorThickness::Default => 2.0,
            SeparatorThickness::Thick => 4.0,
        }
    }
}

/// Label position for labeled separators
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum LabelPosition {
    Start,
    #[default]
    Center,
    End,
}

/// Decorative pattern for separators
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SeparatorPattern {
    #[default]
    Dots,
    Stars,
    Diamonds,
    Arrows,
    Wave,
}
