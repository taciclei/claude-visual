//! Timeline separator for step indicators

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Timeline separator for step indicators
#[derive(IntoElement)]
pub struct TimelineSeparator {
    active: bool,
    completed: bool,
    color: Option<Hsla>,
    active_color: Option<Hsla>,
    length: f32,
    orientation: SeparatorOrientation,
}

impl TimelineSeparator {
    pub fn new() -> Self {
        Self {
            active: false,
            completed: false,
            color: None,
            active_color: None,
            length: 40.0,
            orientation: SeparatorOrientation::Vertical,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn completed(mut self, completed: bool) -> Self {
        self.completed = completed;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn active_color(mut self, color: Hsla) -> Self {
        self.active_color = Some(color);
        self
    }

    pub fn length(mut self, length: f32) -> Self {
        self.length = length;
        self
    }

    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }
}

impl Default for TimelineSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for TimelineSeparator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let inactive_color = self.color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.3,
            a: 1.0,
        });
        let active_color = self.active_color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.5,
            a: 1.0,
        });

        let color = if self.completed || self.active {
            active_color
        } else {
            inactive_color
        };

        match self.orientation {
            SeparatorOrientation::Vertical => div().w(px(2.0)).h(px(self.length)).bg(color),
            SeparatorOrientation::Horizontal => div().h(px(2.0)).w(px(self.length)).bg(color),
        }
    }
}
