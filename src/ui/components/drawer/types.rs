//! Drawer types and enums

use gpui::prelude::*;
use gpui::*;

/// Drawer position/direction
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DrawerPosition {
    /// Left side (default)
    #[default]
    Left,
    /// Right side
    Right,
    /// Top
    Top,
    /// Bottom
    Bottom,
}

/// Drawer size
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DrawerSize {
    /// Small (280px for sides, 200px for top/bottom)
    Small,
    /// Medium (default - 320px / 280px)
    #[default]
    Medium,
    /// Large (480px / 400px)
    Large,
    /// Full screen
    Full,
    /// Custom pixel size
    Custom(f32),
}

impl DrawerSize {
    pub(crate) fn width(&self, position: DrawerPosition) -> Length {
        let is_horizontal = matches!(position, DrawerPosition::Left | DrawerPosition::Right);

        match self {
            DrawerSize::Small => {
                if is_horizontal {
                    px(280.0).into()
                } else {
                    Length::Auto
                }
            }
            DrawerSize::Medium => {
                if is_horizontal {
                    px(320.0).into()
                } else {
                    Length::Auto
                }
            }
            DrawerSize::Large => {
                if is_horizontal {
                    px(480.0).into()
                } else {
                    Length::Auto
                }
            }
            DrawerSize::Full => {
                if is_horizontal {
                    Length::Definite(DefiniteLength::Fraction(1.0))
                } else {
                    Length::Auto
                }
            }
            DrawerSize::Custom(size) => {
                if is_horizontal {
                    px(*size).into()
                } else {
                    Length::Auto
                }
            }
        }
    }

    pub(crate) fn height(&self, position: DrawerPosition) -> Length {
        let is_vertical = matches!(position, DrawerPosition::Top | DrawerPosition::Bottom);

        match self {
            DrawerSize::Small => {
                if is_vertical {
                    px(200.0).into()
                } else {
                    Length::Auto
                }
            }
            DrawerSize::Medium => {
                if is_vertical {
                    px(280.0).into()
                } else {
                    Length::Auto
                }
            }
            DrawerSize::Large => {
                if is_vertical {
                    px(400.0).into()
                } else {
                    Length::Auto
                }
            }
            DrawerSize::Full => {
                if is_vertical {
                    Length::Definite(DefiniteLength::Fraction(1.0))
                } else {
                    Length::Auto
                }
            }
            DrawerSize::Custom(size) => {
                if is_vertical {
                    px(*size).into()
                } else {
                    Length::Auto
                }
            }
        }
    }
}

/// Events emitted by Drawer
#[derive(Debug, Clone)]
pub enum DrawerEvent {
    /// Drawer opened
    Opened,
    /// Drawer closed
    Closed,
    /// Backdrop clicked
    BackdropClicked,
}
