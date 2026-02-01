//! Circular progress indicator

use std::sync::Arc;
use gpui::*;
use gpui::prelude::*;
use crate::app::state::AppState;
use super::types::*;

/// A circular progress indicator (spinner)
pub struct CircularProgress {
    pub(crate) app_state: Arc<AppState>,
    /// Current progress (0.0 to 1.0)
    pub(crate) progress: f32,
    /// Whether progress is indeterminate
    pub(crate) indeterminate: bool,
    /// Size of the spinner
    pub(crate) size: f32,
    /// Stroke width
    pub(crate) stroke_width: f32,
    /// Color
    pub(crate) color: ProgressColor,
}

impl CircularProgress {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            progress: 0.0,
            indeterminate: true,
            size: 24.0,
            stroke_width: 3.0,
            color: ProgressColor::Accent,
        }
    }

    /// Set the progress value
    pub fn set_progress(&mut self, progress: f32, cx: &mut Context<Self>) {
        self.progress = progress.clamp(0.0, 1.0);
        self.indeterminate = false;
        cx.notify();
    }

    /// Set to indeterminate mode
    pub fn set_indeterminate(&mut self, indeterminate: bool, cx: &mut Context<Self>) {
        self.indeterminate = indeterminate;
        cx.notify();
    }

    /// Set size
    pub fn set_size(&mut self, size: f32, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set stroke width
    pub fn set_stroke_width(&mut self, width: f32, cx: &mut Context<Self>) {
        self.stroke_width = width;
        cx.notify();
    }

    /// Set color
    pub fn set_color(&mut self, color: ProgressColor, cx: &mut Context<Self>) {
        self.color = color;
        cx.notify();
    }
}

impl Render for CircularProgress {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        // Determine fill color
        let fill_color = match self.color {
            ProgressColor::Accent => theme.colors.accent,
            ProgressColor::Success => theme.colors.success,
            ProgressColor::Warning => theme.colors.warning,
            ProgressColor::Error => theme.colors.error,
            ProgressColor::Neutral => theme.colors.text_muted,
        };

        // For now, render as a simple container with visual indicator
        // GPUI doesn't have SVG/Canvas support for actual circular progress
        // We'll use a text-based spinner
        let spinner_char = if self.indeterminate {
            "◐" // This would ideally animate through ◐◓◑◒
        } else {
            match (self.progress * 8.0) as u32 {
                0 => "○",
                1 => "◔",
                2 => "◔",
                3 => "◑",
                4 => "◑",
                5 => "◕",
                6 => "◕",
                7 => "◕",
                _ => "●",
            }
        };

        div()
            .id("circular-progress")
            .flex()
            .items_center()
            .justify_center()
            .size(px(self.size))
            .text_color(fill_color)
            .child(
                div()
                    .text_size(px(self.size * 0.8))
                    .child(spinner_char)
            )
    }
}
