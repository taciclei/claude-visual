//! Thumbs up/down feedback component

use gpui::prelude::*;
use gpui::*;

/// Thumbs up/down feedback
#[derive(Clone)]
pub struct ThumbsFeedback {
    pub(crate) value: Option<bool>, // None = no selection, true = up, false = down
}

impl ThumbsFeedback {
    pub fn new() -> Self {
        Self { value: None }
    }

    pub fn up() -> Self {
        Self { value: Some(true) }
    }

    pub fn down() -> Self {
        Self { value: Some(false) }
    }
}

impl Default for ThumbsFeedback {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ThumbsFeedback {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);
        let success = hsla(0.38, 0.7, 0.45, 1.0);
        let error = hsla(0.0, 0.7, 0.5, 1.0);

        let up_selected = self.value == Some(true);
        let down_selected = self.value == Some(false);

        div()
            .flex()
            .items_center()
            .gap_2()
            // Thumbs up
            .child(
                div()
                    .size(px(32.0))
                    .rounded(px(6.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_base()
                    .when(up_selected, |d| {
                        d.bg(success.opacity(0.15)).text_color(success)
                    })
                    .when(!up_selected, |d| {
                        d.text_color(muted)
                            .cursor_pointer()
                            .hover(|s| s.bg(surface_hover).text_color(text))
                    })
                    .child("👍"),
            )
            // Thumbs down
            .child(
                div()
                    .size(px(32.0))
                    .rounded(px(6.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_base()
                    .when(down_selected, |d| {
                        d.bg(error.opacity(0.15)).text_color(error)
                    })
                    .when(!down_selected, |d| {
                        d.text_color(muted)
                            .cursor_pointer()
                            .hover(|s| s.bg(surface_hover).text_color(text))
                    })
                    .child("👎"),
            )
    }
}
