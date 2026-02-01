//! Main video player component

use super::types::{PlaybackState, VideoAspectRatio, VideoPlayerSize, VideoQuality};
use gpui::*;

/// Video player component
#[derive(IntoElement)]
pub struct VideoPlayer {
    pub(super) id: ElementId,
    pub(super) title: Option<SharedString>,
    pub(super) thumbnail: Option<SharedString>,
    pub(super) duration: f64,
    pub(super) current_time: f64,
    pub(super) state: PlaybackState,
    pub(super) volume: f32,
    pub(super) muted: bool,
    pub(super) size: VideoPlayerSize,
    pub(super) aspect_ratio: VideoAspectRatio,
    pub(super) quality: VideoQuality,
    pub(super) show_controls: bool,
    pub(super) fullscreen: bool,
    pub(super) buffered: f32,
}

impl VideoPlayer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            thumbnail: None,
            duration: 0.0,
            current_time: 0.0,
            state: PlaybackState::default(),
            volume: 1.0,
            muted: false,
            size: VideoPlayerSize::default(),
            aspect_ratio: VideoAspectRatio::default(),
            quality: VideoQuality::Auto,
            show_controls: true,
            fullscreen: false,
            buffered: 0.0,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn thumbnail(mut self, thumbnail: impl Into<SharedString>) -> Self {
        self.thumbnail = Some(thumbnail.into());
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

    pub fn state(mut self, state: PlaybackState) -> Self {
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

    pub fn size(mut self, size: VideoPlayerSize) -> Self {
        self.size = size;
        self
    }

    pub fn aspect_ratio(mut self, ratio: VideoAspectRatio) -> Self {
        self.aspect_ratio = ratio;
        self
    }

    pub fn quality(mut self, quality: VideoQuality) -> Self {
        self.quality = quality;
        self
    }

    pub fn show_controls(mut self, show: bool) -> Self {
        self.show_controls = show;
        self
    }

    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn buffered(mut self, buffered: f32) -> Self {
        self.buffered = buffered.clamp(0.0, 1.0);
        self
    }
}
