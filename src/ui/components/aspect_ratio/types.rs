use gpui::*;

/// Common aspect ratios
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Ratio {
    Square,      // 1:1
    #[default]
    Video,       // 16:9
    Classic,     // 4:3
    Portrait,    // 3:4
    Widescreen,  // 21:9
    Photo,       // 3:2
    Golden,      // 1.618:1
    Custom(f32), // Custom ratio (width/height)
}

impl Ratio {
    pub(crate) fn value(&self) -> f32 {
        match self {
            Ratio::Square => 1.0,
            Ratio::Video => 16.0 / 9.0,
            Ratio::Classic => 4.0 / 3.0,
            Ratio::Portrait => 3.0 / 4.0,
            Ratio::Widescreen => 21.0 / 9.0,
            Ratio::Photo => 3.0 / 2.0,
            Ratio::Golden => 1.618,
            Ratio::Custom(r) => *r,
        }
    }
}

/// Image fitting modes
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ImageFit {
    #[default]
    Cover,
    Contain,
    Fill,
}
