//! Collection panel buttons - favorites, pinned messages, recent files

use crate::ui::chat::view::core::ChatView;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    /// Render favorites panel button (⭐ with count)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_favorites_panel_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let fav_count = self.favorite_prompts.len();
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let text_color_available = theme.colors.warning;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_favorites_panel(cx);
        });

        div()
            .id("favorites-panel-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(if self.show_favorites_panel {
                text_color_active
            } else if fav_count > 0 {
                text_color_available
            } else {
                text_color_inactive
            })
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child("⭐")
            .when(fav_count > 0, |d| d.child(format!("{}", fav_count)))
    }

    /// Render pinned messages panel button (📌 with count)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_pinned_panel_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let pin_count = self.pinned_count();
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_pinned_panel(cx);
        });

        div()
            .id("pinned-panel-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(if self.show_pinned_panel {
                text_color_active
            } else if pin_count > 0 {
                text_color_active
            } else {
                text_color_inactive
            })
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child("📌")
            .when(pin_count > 0, |d| d.child(format!("{}", pin_count)))
    }

    /// Render recent files panel button (📂 with count)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_recent_files_panel_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let file_count = self.recent_files_count();
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let text_color_available = theme.colors.info;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_recent_files_panel(cx);
        });

        div()
            .id("recent-files-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(if self.show_recent_files_panel {
                text_color_active
            } else if file_count > 0 {
                text_color_available
            } else {
                text_color_inactive
            })
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child("📂")
            .when(file_count > 0, |d| d.child(format!("{}", file_count)))
    }
}
