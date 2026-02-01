//! Audio player types and enums

/// Playback state
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PlaybackState {
    #[default]
    Stopped,
    Playing,
    Paused,
    Loading,
    Error,
}

/// Audio player size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AudioPlayerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl AudioPlayerSize {
    pub(crate) fn height(&self) -> f32 {
        match self {
            Self::Sm => 40.0,
            Self::Md => 56.0,
            Self::Lg => 72.0,
        }
    }

    pub(crate) fn button_size(&self) -> f32 {
        match self {
            Self::Sm => 28.0,
            Self::Md => 36.0,
            Self::Lg => 48.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            Self::Sm => 11.0,
            Self::Md => 12.0,
            Self::Lg => 14.0,
        }
    }
}

/// Audio player variant
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AudioPlayerVariant {
    #[default]
    Default,
    Minimal,
    Full,
    Waveform,
}
