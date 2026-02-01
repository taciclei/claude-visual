//! Pull to refresh component

use gpui::*;
use gpui::prelude::*;

/// Pull to refresh indicator
#[derive(IntoElement)]
pub struct PullToRefresh {
    pulling: bool,
    refreshing: bool,
    pull_distance: f32,
    threshold: f32,
    pull_text: SharedString,
    release_text: SharedString,
    refreshing_text: SharedString,
    indicator_color: Option<Hsla>,
}

impl PullToRefresh {
    pub fn new() -> Self {
        Self {
            pulling: false,
            refreshing: false,
            pull_distance: 0.0,
            threshold: 60.0,
            pull_text: "Pull to refresh".into(),
            release_text: "Release to refresh".into(),
            refreshing_text: "Refreshing...".into(),
            indicator_color: None,
        }
    }

    pub fn pulling(mut self, pulling: bool) -> Self {
        self.pulling = pulling;
        self
    }

    pub fn refreshing(mut self, refreshing: bool) -> Self {
        self.refreshing = refreshing;
        self
    }

    pub fn pull_distance(mut self, distance: f32) -> Self {
        self.pull_distance = distance;
        self
    }

    pub fn threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn pull_text(mut self, text: impl Into<SharedString>) -> Self {
        self.pull_text = text.into();
        self
    }

    pub fn release_text(mut self, text: impl Into<SharedString>) -> Self {
        self.release_text = text.into();
        self
    }

    pub fn refreshing_text(mut self, text: impl Into<SharedString>) -> Self {
        self.refreshing_text = text.into();
        self
    }

    pub fn indicator_color(mut self, color: Hsla) -> Self {
        self.indicator_color = Some(color);
        self
    }
}

impl Default for PullToRefresh {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for PullToRefresh {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let indicator_color = self.indicator_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.6,
            a: 1.0,
        });

        let (icon, text) = if self.refreshing {
            ("⟳", self.refreshing_text)
        } else if self.pull_distance >= self.threshold {
            ("↓", self.release_text)
        } else {
            ("↓", self.pull_text)
        };

        let height = if self.pulling || self.refreshing {
            self.pull_distance.max(40.0)
        } else {
            0.0
        };

        div()
            .h(px(height))
            .overflow_hidden()
            .flex()
            .items_center()
            .justify_center()
            .gap_2()
            .child(
                div()
                    .text_size(px(16.0))
                    .text_color(indicator_color)
                    .child(icon),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(indicator_color)
                    .child(text),
            )
    }
}
