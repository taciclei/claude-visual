//! Git panel header rendering

use gpui::prelude::*;
use gpui::*;

use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Render git panel header with title and close button
    pub fn render_git_panel_header(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        // Extract listener before div chain
        let toggle_close = cx.listener(|this, _, _window, cx| {
            this.toggle_git_panel(cx);
        });

        // Copy theme colors for move closure
        let surface_hover = theme.colors.surface_hover;
        let text_color = theme.colors.text;

        div()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(theme.colors.border)
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().text_base().child("🔀"))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text)
                            .child("Git Status"),
                    ),
            )
            .child(
                div()
                    .id("close-git-panel")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_sm()
                    .text_color(theme.colors.text_muted)
                    .hover(move |s| s.bg(surface_hover).text_color(text_color))
                    .on_click(toggle_close)
                    .child("×"),
            )
    }
}
