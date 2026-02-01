//! Video player types and enums

pub use crate::ui::components::audio_player::PlaybackState;

/// Video player size preset
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum VideoPlayerSize {
    Sm,
    #[default]
    Md,
    Lg,
    Full,
}

impl VideoPlayerSize {
    pub fn dimensions(&self) -> (f32, f32) {
        match self {
            Self::Sm => (320.0, 180.0),
            Self::Md => (480.0, 270.0),
            Self::Lg => (640.0, 360.0),
            Self::Full => (0.0, 0.0), // Responsive
        }
    }
}

/// Video aspect ratio
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum VideoAspectRatio {
    #[default]
    Widescreen,  // 16:9
    Standard,    // 4:3
    Cinematic,   // 21:9
    Square,      // 1:1
    Portrait,    // 9:16
}

impl VideoAspectRatio {
    pub fn ratio(&self) -> f32 {
        match self {
            Self::Widescreen => 16.0 / 9.0,
            Self::Standard => 4.0 / 3.0,
            Self::Cinematic => 21.0 / 9.0,
            Self::Square => 1.0,
            Self::Portrait => 9.0 / 16.0,
        }
    }
}

/// Video quality option
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoQuality {
    Auto,
    Q360p,
    Q480p,
    Q720p,
    Q1080p,
    Q1440p,
    Q2160p,
}

impl VideoQuality {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Q360p => "360p",
            Self::Q480p => "480p",
            Self::Q720p => "720p",
            Self::Q1080p => "1080p",
            Self::Q1440p => "1440p",
            Self::Q2160p => "4K",
        }
    }
}
