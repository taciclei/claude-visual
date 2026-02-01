//! Utility panel buttons - tips and history

use gpui::*;
use gpui::prelude::*;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Render suggestions toggle button (💡 Tips)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_suggestions_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_suggestions(cx);
            this.update_suggestions(cx);
        });

        div()
            .id("suggestions-toggle-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(if self.show_suggestions { text_color_active } else { text_color_inactive })
            .hover(move |s| s.bg(surface_hover))
            .on_click(on_click)
            .child("💡")
            .when(!self.show_suggestions, |d| d.child("Tips"))
            .when(self.show_suggestions, |d| d.child("Tips ✓"))
    }

    /// Render session history button (📋 History)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_session_history_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let text_muted = theme.colors.text_muted;
        let text = theme.colors.text;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_session_history(cx);
        });

        div()
            .id("session-history-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(text_muted)
            .hover(move |s| s.bg(surface_hover).text_color(text))
            .on_click(on_click)
            .child("📋")
            .child("History")
    }
}
