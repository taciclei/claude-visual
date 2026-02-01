//! Cookie consent banner component

use gpui::prelude::*;
use gpui::*;

/// Cookie consent banner
#[derive(Clone)]
pub struct CookieBanner {
    pub(crate) message: String,
    pub(crate) accept_label: String,
    pub(crate) decline_label: Option<String>,
}

impl CookieBanner {
    pub fn new() -> Self {
        Self {
            message: "We use cookies to enhance your experience.".to_string(),
            accept_label: "Accept".to_string(),
            decline_label: Some("Decline".to_string()),
        }
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn accept_label(mut self, label: impl Into<String>) -> Self {
        self.accept_label = label.into();
        self
    }

    pub fn decline_label(mut self, label: Option<impl Into<String>>) -> Self {
        self.decline_label = label.map(|l| l.into());
        self
    }
}

impl Default for CookieBanner {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for CookieBanner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        div()
            .w_full()
            .p_4()
            .bg(surface)
            .border_t_1()
            .border_color(border)
            .flex()
            .items_center()
            .gap_4()
            // Cookie icon
            .child(div().text_2xl().child("🍪"))
            // Message
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(text)
                    .child(self.message),
            )
            // Buttons
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Decline
                    .when_some(self.decline_label, |d, label| {
                        d.child(
                            div()
                                .px_4()
                                .py_2()
                                .rounded(px(6.0))
                                .border_1()
                                .border_color(border)
                                .text_sm()
                                .text_color(text)
                                .cursor_pointer()
                                .hover(|s| s.bg(surface_hover))
                                .child(label),
                        )
                    })
                    // Accept
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .rounded(px(6.0))
                            .bg(accent)
                            .text_sm()
                            .text_color(gpui::white())
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.9))
                            .child(self.accept_label),
                    ),
            )
    }
}
