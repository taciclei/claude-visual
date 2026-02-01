//! Progress spinner with percentage

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Progress spinner with percentage
#[derive(Clone)]
pub struct ProgressSpinner {
    progress: f32,
    size: SpinnerSize,
    show_percentage: bool,
}

impl ProgressSpinner {
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 100.0),
            size: SpinnerSize::Large,
            show_percentage: true,
        }
    }

    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }
}

impl RenderOnce for ProgressSpinner {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let track = hsla(0.0, 0.0, 0.2, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);

        let size = self.size.size();

        div()
            .size(px(size))
            .relative()
            .flex()
            .items_center()
            .justify_center()
            // Background ring
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .rounded_full()
                    .border(px(self.size.stroke_width()))
                    .border_color(track)
            )
            // Progress ring (simplified visual)
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .rounded_full()
                    .border(px(self.size.stroke_width()))
                    .border_color(hsla(0.0, 0.0, 0.0, 0.0))
                    .border_color(accent)
                    .when(self.progress >= 25.0, |d| d.border_color(accent))
                    .when(self.progress >= 50.0, |d| d.border_color(accent))
                    .when(self.progress >= 75.0, |d| d.border_color(accent))
            )
            // Percentage text
            .when(self.show_percentage, |d| {
                d.child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text)
                        .child(format!("{}%", self.progress as u32))
                )
            })
    }
}
