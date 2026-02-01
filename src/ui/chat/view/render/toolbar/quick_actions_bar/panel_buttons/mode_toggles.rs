//! Mode toggle buttons - focus mode

use gpui::*;
use gpui::prelude::*;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Render focus mode toggle button (🎯/○)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_focus_mode_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let accent_bg = theme.colors.accent.opacity(0.1);
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_focus_mode(cx);
        });

        div()
            .id("focus-mode-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(if self.focus_mode { text_color_active } else { text_color_inactive })
            .when(self.focus_mode, move |d| d.bg(accent_bg))
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child(if self.focus_mode { "🎯" } else { "○" })
    }
}
