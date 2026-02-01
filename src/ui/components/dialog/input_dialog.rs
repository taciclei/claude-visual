//! Input dialog component (prompt for text)

use gpui::*;
use gpui::prelude::*;

/// Input dialog (prompt for text)
#[derive(Clone)]
pub struct InputDialog {
    title: String,
    message: Option<String>,
    placeholder: String,
    default_value: String,
    confirm_label: String,
    cancel_label: String,
}

impl InputDialog {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: None,
            placeholder: "Enter value...".to_string(),
            default_value: String::new(),
            confirm_label: "OK".to_string(),
            cancel_label: "Cancel".to_string(),
        }
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = value.into();
        self
    }

    pub fn confirm_label(mut self, label: impl Into<String>) -> Self {
        self.confirm_label = label.into();
        self
    }

    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }
}

impl RenderOnce for InputDialog {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.2, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let backdrop = hsla(0.0, 0.0, 0.0, 0.6);

        div()
            .absolute()
            .inset_0()
            .bg(backdrop)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(400.0))
                    .bg(surface)
                    .rounded(px(12.0))
                    .border_1()
                    .border_color(border)
                    .shadow_xl()
                    .flex()
                    .flex_col()
                    // Header
                    .child(
                        div()
                            .w_full()
                            .px_5()
                            .pt_5()
                            .pb_3()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(text)
                                    .child(self.title)
                            )
                            .when_some(self.message, |d, msg| {
                                d.child(
                                    div()
                                        .text_sm()
                                        .text_color(text_muted)
                                        .child(msg)
                                )
                            })
                    )
                    // Input field
                    .child(
                        div()
                            .w_full()
                            .px_5()
                            .child(
                                div()
                                    .w_full()
                                    .px_3()
                                    .py_2()
                                    .bg(hsla(0.0, 0.0, 0.1, 1.0))
                                    .rounded(px(6.0))
                                    .border_1()
                                    .border_color(border)
                                    .text_sm()
                                    .text_color(if self.default_value.is_empty() { text_muted } else { text })
                                    .child(if self.default_value.is_empty() {
                                        self.placeholder
                                    } else {
                                        self.default_value
                                    })
                            )
                    )
                    // Buttons
                    .child(
                        div()
                            .w_full()
                            .px_5()
                            .py_4()
                            .flex()
                            .items_center()
                            .justify_end()
                            .gap_2()
                            .child(
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
                                    .child(self.cancel_label)
                            )
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
                                    .child(self.confirm_label)
                            )
                    )
            )
    }
}
