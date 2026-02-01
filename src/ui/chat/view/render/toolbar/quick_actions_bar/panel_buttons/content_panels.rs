//! Content management panel buttons - context, export, notes

use gpui::*;
use gpui::prelude::*;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Render context panel button (📚 Context with file count and usage indicator)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_context_panel_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let file_count = self.context_file_count();
        let usage_pct = self.context_usage_percentage();
        let usage_color = if usage_pct < 0.5 {
            theme.colors.success
        } else if usage_pct < 0.8 {
            theme.colors.warning
        } else {
            theme.colors.error
        };

        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_context_panel(cx);
        });

        div()
            .id("context-panel-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(
                if self.show_context_panel {
                    text_color_active
                } else if self.context_used > 0 {
                    usage_color
                } else {
                    text_color_inactive
                }
            )
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child("📚")
            .when(file_count > 0, |d| d.child(format!("{}", file_count)))
            .when(file_count == 0, |d| d.child("Context"))
    }

    /// Render export panel button (💾 Export)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_export_panel_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let has_msgs = self.has_messages();
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let text_color_available = theme.colors.text;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.request_export(cx);
        });

        div()
            .id("export-panel-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(
                if self.show_export_panel {
                    text_color_active
                } else if has_msgs {
                    text_color_available
                } else {
                    text_color_inactive
                }
            )
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child("💾")
            .child("Export")
    }

    /// Render notes panel button (📝 Notes)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_notes_panel_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let has_notes = self.has_notes();
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let text_color_available = theme.colors.text;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_notes_panel(cx);
        });

        div()
            .id("notes-panel-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(
                if self.show_notes_panel {
                    text_color_active
                } else if has_notes {
                    text_color_available
                } else {
                    text_color_inactive
                }
            )
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child("📝")
            .when(has_notes, |d| d.child("Notes"))
    }
}
