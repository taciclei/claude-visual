//! Inline loading indicator for text

use super::spinner::Spinner;
use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Inline loading indicator for text
#[derive(Clone)]
pub struct InlineLoader {
    text: String,
}

impl InlineLoader {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for InlineLoader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                Spinner::new()
                    .size(SpinnerSize::XSmall)
                    .variant(SpinnerVariant::Dots),
            )
            .child(div().text_sm().text_color(text_muted).child(self.text))
    }
}
