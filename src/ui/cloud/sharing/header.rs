//! Share dialog header rendering

use gpui::prelude::*;
use gpui::*;

use super::dialog::ShareDialog;
use super::types::ShareDialogEvent;

impl ShareDialog {
    /// Render header
    pub(super) fn render_header(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .pb_4()
            .border_b_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.colors.text)
                            .child("Share Conversation"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .overflow_hidden()
                            .child(self.conversation_title.clone()),
                    ),
            )
            .child(
                div()
                    .id("close-share-dialog")
                    .size_6()
                    .rounded_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|this| this.bg(theme.colors.surface_hover))
                    .on_click(cx.listener(|_this, _event, _window, cx| {
                        cx.emit(ShareDialogEvent::Closed);
                    }))
                    .child(
                        div()
                            .text_lg()
                            .text_color(theme.colors.text_muted)
                            .child("×"),
                    ),
            )
    }
}
