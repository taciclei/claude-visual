//! URL display component

use gpui::*;
use gpui::prelude::*;

/// URL display
#[derive(Clone)]
pub struct UrlDisplay {
    url: String,
    truncate: bool,
    clickable: bool,
}

impl UrlDisplay {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            truncate: true,
            clickable: true,
        }
    }

    pub fn full(mut self) -> Self {
        self.truncate = false;
        self
    }

    pub fn not_clickable(mut self) -> Self {
        self.clickable = false;
        self
    }

    pub(crate) fn display_url(&self) -> String {
        if !self.truncate {
            return self.url.clone();
        }

        // Strip protocol
        let url = self.url
            .trim_start_matches("https://")
            .trim_start_matches("http://");

        // Truncate if too long
        if url.len() > 40 {
            format!("{}...{}", &url[..20], &url[url.len()-15..])
        } else {
            url.to_string()
        }
    }
}

impl RenderOnce for UrlDisplay {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        let display = self.display_url();

        let mut container = div()
            .flex()
            .items_center()
            .gap_1()
            .text_sm();

        if self.clickable {
            container = container
                .text_color(accent)
                .cursor_pointer()
                .hover(|s| s.underline());
        } else {
            container = container.text_color(text_muted);
        }

        container
            .child("🔗")
            .child(display)
    }
}
