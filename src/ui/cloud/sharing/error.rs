//! Error message rendering

use gpui::prelude::*;
use gpui::*;

use super::dialog::ShareDialog;

impl ShareDialog {
    /// Render error message
    pub(super) fn render_error(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div().when_some(self.error_message.clone(), |this, message| {
            this.child(
                div()
                    .w_full()
                    .p_3()
                    .rounded_md()
                    .bg(theme.colors.error.opacity(0.1))
                    .border_1()
                    .border_color(theme.colors.error.opacity(0.3))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.error)
                            .child(message),
                    ),
            )
        })
    }
}
