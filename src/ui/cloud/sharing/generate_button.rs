//! Generate button rendering

use gpui::prelude::*;
use gpui::*;

use super::dialog::ShareDialog;

impl ShareDialog {
    /// Render generate button
    pub(super) fn render_generate_button(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .id("generate-link")
            .w_full()
            .py_2()
            .rounded_md()
            .cursor_pointer()
            .bg(if self.is_generating {
                theme.colors.surface
            } else {
                theme.colors.accent
            })
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .text_color(if self.is_generating {
                theme.colors.text_muted
            } else {
                theme.colors.background
            })
            .text_center()
            .when(!self.is_generating, |this| {
                this.hover(|this| this.opacity(0.9))
            })
            .when(self.is_generating, |this| this.cursor_not_allowed())
            .on_click(cx.listener(|this, _event, _window, cx| {
                if !this.is_generating {
                    this.generate_link(cx);
                }
            }))
            .child(if self.is_generating {
                "Generating..."
            } else {
                "Generate Link"
            })
    }
}
