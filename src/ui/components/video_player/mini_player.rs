//! Picture-in-picture mini player

use super::types::PlaybackState;
use gpui::*;

/// Picture-in-picture mini player
#[derive(IntoElement)]
pub struct MiniPlayer {
    pub(super) id: ElementId,
    pub(super) title: SharedString,
    pub(super) thumbnail: Option<SharedString>,
    pub(super) state: PlaybackState,
    pub(super) progress: f32,
}

impl MiniPlayer {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            thumbnail: None,
            state: PlaybackState::default(),
            progress: 0.0,
        }
    }

    pub fn thumbnail(mut self, thumbnail: impl Into<SharedString>) -> Self {
        self.thumbnail = Some(thumbnail.into());
        self
    }

    pub fn state(mut self, state: PlaybackState) -> Self {
        self.state = state;
        self
    }

    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }
}
