//! Stateless empty placeholder component

use gpui::prelude::*;
use gpui::*;

/// Stateless empty placeholder
#[derive(Clone)]
pub struct EmptyPlaceholder {
    icon: String,
    message: String,
}

impl EmptyPlaceholder {
    pub fn new(icon: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            message: message.into(),
        }
    }

    pub fn no_results() -> Self {
        Self::new("🔍", "No results found")
    }

    pub fn no_data() -> Self {
        Self::new("📭", "No data yet")
    }

    pub fn no_messages() -> Self {
        Self::new("💬", "No messages")
    }

    pub fn loading() -> Self {
        Self::new("⏳", "Loading...")
    }
}

impl RenderOnce for EmptyPlaceholder {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        div()
            .w_full()
            .py_8()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_2()
            .child(div().text_size(px(32.0)).child(self.icon))
            .child(div().text_sm().text_color(text_muted).child(self.message))
    }
}
