//! Video thumbnail component

use gpui::*;

/// Video thumbnail component
#[derive(IntoElement)]
pub struct VideoThumbnail {
    pub(super) id: ElementId,
    pub(super) thumbnail: Option<SharedString>,
    pub(super) duration: f64,
    pub(super) title: Option<SharedString>,
    pub(super) width: f32,
    pub(super) show_play_icon: bool,
    pub(super) progress: Option<f32>,
}

impl VideoThumbnail {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            thumbnail: None,
            duration: 0.0,
            title: None,
            width: 240.0,
            show_play_icon: true,
            progress: None,
        }
    }

    pub fn thumbnail(mut self, thumbnail: impl Into<SharedString>) -> Self {
        self.thumbnail = Some(thumbnail.into());
        self
    }

    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn show_play_icon(mut self, show: bool) -> Self {
        self.show_play_icon = show;
        self
    }

    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = Some(progress.clamp(0.0, 1.0));
        self
    }
}
