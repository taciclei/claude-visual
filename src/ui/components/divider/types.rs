//! Divider types and enums

/// Divider orientation
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DividerOrientation {
    /// Horizontal divider
    #[default]
    Horizontal,
    /// Vertical divider
    Vertical,
}

/// Divider style
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DividerStyle {
    /// Solid line
    #[default]
    Solid,
    /// Dashed line (simulated with shorter width)
    Dashed,
    /// Dotted line (simulated)
    Dotted,
}

/// Divider thickness
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DividerThickness {
    /// Thin (1px)
    Thin,
    /// Normal (1px with more opacity)
    #[default]
    Normal,
    /// Thick (2px)
    Thick,
}

impl DividerThickness {
    pub(crate) fn pixels(&self) -> f32 {
        match self {
            DividerThickness::Thin => 1.0,
            DividerThickness::Normal => 1.0,
            DividerThickness::Thick => 2.0,
        }
    }

    pub(crate) fn opacity(&self) -> f32 {
        match self {
            DividerThickness::Thin => 0.3,
            DividerThickness::Normal => 0.5,
            DividerThickness::Thick => 0.7,
        }
    }
}
