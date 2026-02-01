//! Progress bar component

use std::sync::Arc;
use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;
use crate::app::state::AppState;
use super::types::*;

/// Progress bar component
pub struct ProgressBar {
    pub(crate) app_state: Arc<AppState>,
    /// Current progress (0.0 to 1.0)
    pub(crate) progress: f32,
    /// Whether progress is indeterminate
    pub(crate) indeterminate: bool,
    /// Style variant
    pub(crate) style: ProgressStyle,
    /// Color scheme
    pub(crate) color: ProgressColor,
    /// Whether to show percentage label
    pub(crate) show_label: bool,
    /// Custom label (overrides percentage)
    pub(crate) custom_label: Option<String>,
    /// Height of the bar
    pub(crate) height: f32,
    /// Whether to animate (for striped/indeterminate)
    pub(crate) animated: bool,
}

impl ProgressBar {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            progress: 0.0,
            indeterminate: false,
            style: ProgressStyle::Default,
            color: ProgressColor::Accent,
            show_label: false,
            custom_label: None,
            height: 8.0,
            animated: true,
        }
    }

    /// Set the progress value (0.0 to 1.0)
    pub fn set_progress(&mut self, progress: f32, cx: &mut Context<Self>) {
        self.progress = progress.clamp(0.0, 1.0);
        self.indeterminate = false;
        cx.notify();
    }

    /// Set to indeterminate mode (loading spinner style)
    pub fn set_indeterminate(&mut self, indeterminate: bool, cx: &mut Context<Self>) {
        self.indeterminate = indeterminate;
        cx.notify();
    }

    /// Set the style variant
    pub fn set_style(&mut self, style: ProgressStyle, cx: &mut Context<Self>) {
        self.style = style;
        cx.notify();
    }

    /// Set the color scheme
    pub fn set_color(&mut self, color: ProgressColor, cx: &mut Context<Self>) {
        self.color = color;
        cx.notify();
    }

    /// Show/hide percentage label
    pub fn set_show_label(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_label = show;
        cx.notify();
    }

    /// Set a custom label
    pub fn set_label(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.custom_label = label;
        cx.notify();
    }

    /// Set the height
    pub fn set_height(&mut self, height: f32, cx: &mut Context<Self>) {
        self.height = height;
        cx.notify();
    }

    /// Enable/disable animation
    pub fn set_animated(&mut self, animated: bool, cx: &mut Context<Self>) {
        self.animated = animated;
        cx.notify();
    }

    /// Get current progress
    pub fn progress(&self) -> f32 {
        self.progress
    }

    /// Check if indeterminate
    pub fn is_indeterminate(&self) -> bool {
        self.indeterminate
    }

    /// Get the label text
    fn label_text(&self) -> String {
        if let Some(label) = &self.custom_label {
            label.clone()
        } else {
            format!("{}%", (self.progress * 100.0) as u32)
        }
    }
}

impl Render for ProgressBar {
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

        // Determine track color
        let track_color = theme.colors.surface;

        // Calculate width percentage
        let width_percent = if self.indeterminate {
            30.0 // Indeterminate shows a sliding bar
        } else {
            self.progress * 100.0
        };

        // Calculate border radius
        let radius = match self.style {
            ProgressStyle::Rounded | ProgressStyle::Default => self.height / 2.0,
            ProgressStyle::Thin => 2.0,
            ProgressStyle::Striped => self.height / 2.0,
        };

        // Determine height
        let bar_height = match self.style {
            ProgressStyle::Thin => 3.0,
            _ => self.height,
        };

        let label_text = self.label_text();

        div()
            .id("progress-bar")
            .w_full()
            .flex()
            .flex_col()
            .gap_1()
            // Label row (if shown)
            .when(self.show_label, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .child(
                            div()
                                .when(!self.indeterminate, |d| d.child(label_text.clone()))
                                .when(self.indeterminate, |d| d.child("Loading..."))
                        )
                )
            })
            // Progress bar track
            .child(
                div()
                    .w_full()
                    .h(px(bar_height))
                    .rounded(px(radius))
                    .bg(track_color)
                    .border_1()
                    .border_color(theme.colors.border)
                    .overflow_hidden()
                    .relative()
                    // Progress fill
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .bottom_0()
                            .left_0()
                            .when(!self.indeterminate, |d| d.w(pct(width_percent)))
                            .when(self.indeterminate, |d| {
                                // Indeterminate: show a sliding bar in the middle
                                d.w(pct(30.0)).left(pct(35.0))
                            })
                            .bg(fill_color)
                            .rounded(px(radius.max(1.0) - 1.0))
                    )
            )
    }
}

impl EventEmitter<ProgressBarEvent> for ProgressBar {}
