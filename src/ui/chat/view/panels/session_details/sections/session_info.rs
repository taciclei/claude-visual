//! Session info section rendering

use gpui::prelude::*;
use gpui::*;

use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Renders the session info section
    pub(crate) fn render_session_info_section(
        &self,
        info: &crate::claude::message::SessionInfo,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.text_muted)
                    .child("SESSION INFO"),
            )
            .child(
                div()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(theme.colors.background)
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(self.render_detail_row_with_copy(
                        "Session ID",
                        &info.session_id,
                        "session-id",
                        &theme,
                        cx,
                    ))
                    .child(self.render_detail_row("Model", &info.model, &theme))
                    .child(self.render_detail_row("Version", &info.version, &theme))
                    .child(self.render_detail_row_with_copy("CWD", &info.cwd, "cwd", &theme, cx)),
            )
    }
}
