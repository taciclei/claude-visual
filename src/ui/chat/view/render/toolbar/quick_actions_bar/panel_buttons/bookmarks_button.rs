//! Bookmarks filter button

use crate::ui::chat::view::core::ChatView;
use gpui::*;

impl ChatView {
    /// Render bookmarks filter button (⭐ with count, only shown when bookmarks exist)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_bookmarks_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Option<Stateful<Div>> {
        let count = self.bookmarked_count();
        if count == 0 {
            return None;
        }

        let is_filtered = self.show_bookmarks_only;
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_bookmarked_only(cx);
        });

        Some(
            div()
                .id("bookmarks-btn")
                .flex()
                .items_center()
                .gap_1()
                .px_2()
                .py(px(2.0))
                .rounded_md()
                .cursor_pointer()
                .text_xs()
                .text_color(if is_filtered {
                    text_color_active
                } else {
                    text_color_inactive
                })
                .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
                .on_click(on_click)
                .child("⭐")
                .child(format!("{}", count)),
        )
    }
}
