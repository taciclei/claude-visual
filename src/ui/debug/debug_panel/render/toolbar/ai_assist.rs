//! AI assistant button

use gpui::prelude::*;
use gpui::*;

use crate::ui::debug::debug_panel::DebugPanel;

impl DebugPanel {
    /// Render AI assistant button
    pub(super) fn render_ai_assist_button(
        &self,
        surface_color: Hsla,
        accent_color: Hsla,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        // Extract listener before div chain
        let on_ai_assist = cx.listener(|this, _, _window, cx| {
            this.toggle_ai_menu(cx);
        });

        div()
            .id("debug-ai-assist")
            .px_2()
            .py_1()
            .rounded_md()
            .cursor_pointer()
            .bg(if self.show_ai_menu {
                accent_color.opacity(0.2)
            } else {
                surface_color
            })
            .hover(|s| s.bg(accent_color.opacity(0.2)))
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .text_color(accent_color)
            .on_click(on_ai_assist)
            .child("AI Help")
    }
}
