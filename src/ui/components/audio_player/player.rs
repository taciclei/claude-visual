//! AudioPlayer struct and builder methods

use gpui::*;

/// Audio player component
#[derive(IntoElement)]
pub struct AudioPlayer {
    pub(super) id: ElementId,
    pub(super) title: Option<SharedString>,
    pub(super) artist: Option<SharedString>,
    pub(super) duration: f64,
    pub(super) current_time: f64,
    pub(super) state: super::types::PlaybackState,
    pub(super) volume: f32,
    pub(super) muted: bool,
    pub(super) size: super::types::AudioPlayerSize,
    pub(super) variant: super::types::AudioPlayerVariant,
    pub(super) show_time: bool,
    pub(super) show_volume: bool,
}

impl AudioPlayer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            artist: None,
            duration: 0.0,
            current_time: 0.0,
            state: super::types::PlaybackState::default(),
            volume: 1.0,
            muted: false,
            size: super::types::AudioPlayerSize::default(),
            variant: super::types::AudioPlayerVariant::default(),
            show_time: true,
            show_volume: true,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn artist(mut self, artist: impl Into<SharedString>) -> Self {
        self.artist = Some(artist.into());
        self
    }

    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }

    pub fn current_time(mut self, time: f64) -> Self {
        self.current_time = time;
        self
    }

    pub fn state(mut self, state: super::types::PlaybackState) -> Self {
        self.state = state;
        self
    }

    pub fn volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 1.0);
        self
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    pub fn size(mut self, size: super::types::AudioPlayerSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: super::types::AudioPlayerVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn show_time(mut self, show: bool) -> Self {
        self.show_time = show;
        self
    }

    pub fn show_volume(mut self, show: bool) -> Self {
        self.show_volume = show;
        self
    }

    pub(crate) fn format_time(seconds: f64) -> String {
        let mins = (seconds / 60.0).floor() as i32;
        let secs = (seconds % 60.0).floor() as i32;
        format!("{}:{:02}", mins, secs)
    }
}
