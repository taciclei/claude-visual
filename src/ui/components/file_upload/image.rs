//! Image upload component

use gpui::prelude::*;
use gpui::*;

/// Image upload with preview
#[derive(Clone)]
pub struct ImageUpload {
    /// Current image URL/path
    current_image: Option<String>,
    /// Placeholder text
    placeholder: String,
    /// Aspect ratio
    aspect_ratio: Option<f32>,
    /// Circular crop
    circular: bool,
}

impl ImageUpload {
    pub fn new() -> Self {
        Self {
            current_image: None,
            placeholder: "Upload image".to_string(),
            aspect_ratio: None,
            circular: false,
        }
    }

    pub fn current(mut self, image_url: impl Into<String>) -> Self {
        self.current_image = Some(image_url.into());
        self
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.aspect_ratio = Some(ratio);
        self
    }

    pub fn circular(mut self) -> Self {
        self.circular = true;
        self
    }
}

impl Default for ImageUpload {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ImageUpload {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        let size = px(120.0);

        let mut container = div()
            .size(size)
            .bg(surface)
            .border_2()
            .border_color(border)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_2()
            .cursor_pointer()
            .hover(|s| s.border_color(hsla(0.6, 0.8, 0.6, 1.0)));

        if self.circular {
            container = container.rounded_full();
        } else {
            container = container.rounded(px(8.0));
        }

        if self.current_image.is_some() {
            // Show image placeholder (actual image would need asset loading)
            container = container.child(div().text_3xl().child("🖼️")).child(
                div()
                    .absolute()
                    .inset_0()
                    .bg(hsla(0.0, 0.0, 0.0, 0.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .hover(|s| s.bg(hsla(0.0, 0.0, 0.0, 0.5)))
                    .child(
                        div()
                            .opacity(0.0)
                            .hover(|s| s.opacity(1.0))
                            .text_sm()
                            .text_color(gpui::white())
                            .child("Change"),
                    ),
            );
        } else {
            container = container
                .child(div().text_2xl().text_color(text_muted).child("📷"))
                .child(
                    div()
                        .text_xs()
                        .text_color(text_muted)
                        .child(self.placeholder),
                );
        }

        container.relative()
    }
}
