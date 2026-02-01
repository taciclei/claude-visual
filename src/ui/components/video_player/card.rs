//! Video card with metadata

use gpui::*;

/// Video card with metadata
#[derive(IntoElement)]
pub struct VideoCard {
    pub(super) id: ElementId,
    pub(super) thumbnail: Option<SharedString>,
    pub(super) title: SharedString,
    pub(super) channel: SharedString,
    pub(super) views: u64,
    pub(super) uploaded_at: SharedString,
    pub(super) duration: f64,
    pub(super) width: f32,
}

impl VideoCard {
    pub fn new(
        id: impl Into<ElementId>,
        title: impl Into<SharedString>,
        channel: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            thumbnail: None,
            title: title.into(),
            channel: channel.into(),
            views: 0,
            uploaded_at: "".into(),
            duration: 0.0,
            width: 320.0,
        }
    }

    pub fn thumbnail(mut self, thumbnail: impl Into<SharedString>) -> Self {
        self.thumbnail = Some(thumbnail.into());
        self
    }

    pub fn views(mut self, views: u64) -> Self {
        self.views = views;
        self
    }

    pub fn uploaded_at(mut self, at: impl Into<SharedString>) -> Self {
        self.uploaded_at = at.into();
        self
    }

    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}
