use gpui::*;

/// Watermark position on the container
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum WatermarkPosition {
    #[default]
    Center,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    TopCenter,
    BottomCenter,
    LeftCenter,
    RightCenter,
    Tiled,
}

/// Watermark variant style
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum WatermarkVariant {
    #[default]
    Text,
    Image,
    Pattern,
    Diagonal,
}

/// Stamp type with predefined colors
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StampType {
    #[default]
    Draft,
    Confidential,
    Approved,
    Rejected,
    Review,
    Final,
    Sample,
    Copy,
    Custom,
}

impl StampType {
    pub(crate) fn color(&self) -> gpui::Hsla {
        match self {
            Self::Draft => rgba(0x6b7280ff).into(),
            Self::Confidential => rgba(0xef4444ff).into(),
            Self::Approved => rgba(0x22c55eff).into(),
            Self::Rejected => rgba(0xef4444ff).into(),
            Self::Review => rgba(0xf59e0bff).into(),
            Self::Final => rgba(0x3b82f6ff).into(),
            Self::Sample => rgba(0xa855f7ff).into(),
            Self::Copy => rgba(0x6b7280ff).into(),
            Self::Custom => rgba(0x888888ff).into(),
        }
    }

    pub(crate) fn default_text(&self) -> &str {
        match self {
            Self::Draft => "DRAFT",
            Self::Confidential => "CONFIDENTIAL",
            Self::Approved => "APPROVED",
            Self::Rejected => "REJECTED",
            Self::Review => "UNDER REVIEW",
            Self::Final => "FINAL",
            Self::Sample => "SAMPLE",
            Self::Copy => "COPY",
            Self::Custom => "",
        }
    }
}

/// Stamp size preset
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StampSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl StampSize {
    pub(crate) fn font_size(&self) -> f32 {
        match self {
            Self::Sm => 24.0,
            Self::Md => 36.0,
            Self::Lg => 48.0,
            Self::Xl => 72.0,
        }
    }

    pub(crate) fn padding(&self) -> f32 {
        match self {
            Self::Sm => 8.0,
            Self::Md => 12.0,
            Self::Lg => 16.0,
            Self::Xl => 24.0,
        }
    }
}

/// Pattern type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PatternType {
    #[default]
    Dots,
    DiagonalStripes,
    HorizontalStripes,
    VerticalStripes,
    Grid,
    Checkerboard,
    Crosshatch,
}
