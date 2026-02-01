//! Input-related panel buttons - file picker, commands, templates

use gpui::*;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Render file picker button (@ Files)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_file_picker_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_file_picker(cx);
        });

        div()
            .id("file-picker-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(if self.file_picker.visible { text_color_active } else { text_color_inactive })
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child("@")
            .child("Files")
    }

    /// Render commands panel button (/ Commands)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_commands_panel_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_commands_panel(cx);
        });

        div()
            .id("commands-panel-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(if self.show_commands_panel { text_color_active } else { text_color_inactive })
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child("/")
            .child("Commands")
    }

    /// Render templates panel button (📝 Templates)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_templates_panel_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_templates_panel(cx);
        });

        div()
            .id("templates-panel-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(if self.show_templates_panel { text_color_active } else { text_color_inactive })
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child("📝")
            .child("Templates")
    }
}
