use gpui::*;

/// Heat map color scale
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum HeatMapScale {
    #[default]
    Green,
    Blue,
    Purple,
    Orange,
    Red,
    Gray,
    Custom,
}

impl HeatMapScale {
    pub(crate) fn colors(&self) -> Vec<gpui::Hsla> {
        match self {
            Self::Green => vec![
                rgba(0xebedf0ff).into(), // Empty
                rgba(0x9be9a8ff).into(), // Level 1
                rgba(0x40c463ff).into(), // Level 2
                rgba(0x30a14eff).into(), // Level 3
                rgba(0x216e39ff).into(), // Level 4
            ],
            Self::Blue => vec![
                rgba(0xebedf0ff).into(),
                rgba(0xc6e6ffff).into(),
                rgba(0x79c0ffff).into(),
                rgba(0x3b82f6ff).into(),
                rgba(0x1d4ed8ff).into(),
            ],
            Self::Purple => vec![
                rgba(0xebedf0ff).into(),
                rgba(0xe9d5ffff).into(),
                rgba(0xc084fcff).into(),
                rgba(0xa855f7ff).into(),
                rgba(0x7c3aedff).into(),
            ],
            Self::Orange => vec![
                rgba(0xebedf0ff).into(),
                rgba(0xfed7aaff).into(),
                rgba(0xfb923cff).into(),
                rgba(0xf97316ff).into(),
                rgba(0xea580cff).into(),
            ],
            Self::Red => vec![
                rgba(0xebedf0ff).into(),
                rgba(0xfecacaff).into(),
                rgba(0xf87171ff).into(),
                rgba(0xef4444ff).into(),
                rgba(0xdc2626ff).into(),
            ],
            Self::Gray => vec![
                rgba(0xebedf0ff).into(),
                rgba(0xd4d4d4ff).into(),
                rgba(0xa3a3a3ff).into(),
                rgba(0x737373ff).into(),
                rgba(0x525252ff).into(),
            ],
            Self::Custom => vec![
                rgba(0xebedf0ff).into(),
                rgba(0xddddddff).into(),
                rgba(0xbbbbbbff).into(),
                rgba(0x999999ff).into(),
                rgba(0x777777ff).into(),
            ],
        }
    }

    pub(crate) fn color_for_level(&self, level: usize) -> gpui::Hsla {
        let colors = self.colors();
        colors.get(level.min(colors.len() - 1)).copied().unwrap_or(colors[0])
    }
}

/// Heat map cell data
#[derive(Debug, Clone)]
pub struct HeatMapCell {
    pub value: f32,
    pub label: Option<SharedString>,
    pub date: Option<SharedString>,
}

impl HeatMapCell {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            label: None,
            date: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_date(mut self, date: impl Into<SharedString>) -> Self {
        self.date = Some(date.into());
        self
    }
}
