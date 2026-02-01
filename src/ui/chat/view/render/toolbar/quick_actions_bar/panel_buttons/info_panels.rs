//! Information panel buttons - statistics and tags

use crate::ui::chat::view::core::ChatView;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    /// Render statistics panel button (📊)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_stats_panel_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_stats_panel(cx);
        });

        div()
            .id("stats-panel-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(if self.show_stats_panel {
                text_color_active
            } else {
                text_color_inactive
            })
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child("📊")
    }

    /// Render tags editor button (🏷️ with count)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_tags_panel_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let tag_count = self.conversation_tags.len();
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let text_color_available = theme.colors.text;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_tags_editor(cx);
        });

        div()
            .id("tags-panel-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(if self.show_tags_editor {
                text_color_active
            } else if tag_count > 0 {
                text_color_available
            } else {
                text_color_inactive
            })
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child("🏷️")
            .when(tag_count > 0, |d| d.child(format!("{}", tag_count)))
    }
}
