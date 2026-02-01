//! Error state rendering for update notifications

use gpui::prelude::*;
use gpui::*;

use super::super::core::UpdateNotification;
use super::super::types::{SimpleColors, UpdateNotificationEvent};

impl UpdateNotification {
    /// Render error state
    pub(crate) fn render_error(
        &self,
        error: &str,
        theme: &SimpleColors,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        // Extract listener
        let on_dismiss = cx.listener(|this, _, _window, cx| {
            this.dismiss(cx);
            cx.emit(UpdateNotificationEvent::Dismiss);
        });

        // Copy theme colors for closures
        let warning = theme.warning;
        let background = theme.background;
        let text = theme.text;
        let text_muted = theme.text_muted;
        let surface_hover = theme.surface_hover;

        div()
            .w_full()
            .bg(warning.opacity(0.1))
            .border_1()
            .border_color(warning.opacity(0.3))
            .rounded_md()
            .p_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .w_6()
                                    .h_6()
                                    .rounded_full()
                                    .bg(warning)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(div().text_color(background).text_xs().child("!")),
                            )
                            .child(
                                div()
                                    .text_color(text)
                                    .text_sm()
                                    .child(format!("Update check failed: {}", error)),
                            ),
                    )
                    .child(
                        div()
                            .id("dismiss-error-button")
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .text_color(text_muted)
                            .text_sm()
                            .hover(|s| s.bg(surface_hover))
                            .on_click(on_dismiss)
                            .child("Dismiss"),
                    ),
            )
    }
}
