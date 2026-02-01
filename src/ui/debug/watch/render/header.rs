//! Watch view header rendering

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use super::super::core::WatchView;
use super::super::events::WatchViewEvent;

impl WatchView {
    pub fn render_header(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.count();
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let accent = theme.colors.accent;
        let border = theme.colors.border;
        let surface = theme.colors.surface;

        let on_refresh_all = cx.listener(|_this, _, _window, cx| {
            cx.emit(WatchViewEvent::RefreshAll);
        });

        let on_add = cx.listener(|this, _, _window, cx| {
            this.start_adding(cx);
        });

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_2()
            .py_1()
            .bg(surface)
            .border_b_1()
            .border_color(border)
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(text_color)
                    .child(format!("Watch ({})", count)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    // Refresh all button
                    .child(
                        div()
                            .id("refresh-watches")
                            .px_2()
                            .py_0p5()
                            .rounded_sm()
                            .text_xs()
                            .text_color(text_muted)
                            .cursor_pointer()
                            .hover(|s| s.bg(border))
                            .on_click(on_refresh_all)
                            .child("↻"),
                    )
                    // Add button
                    .child(
                        div()
                            .id("add-watch")
                            .px_2()
                            .py_0p5()
                            .rounded_sm()
                            .text_xs()
                            .text_color(accent)
                            .cursor_pointer()
                            .hover(|s| s.bg(accent.opacity(0.1)))
                            .on_click(on_add)
                            .child("+ Add"),
                    ),
            )
    }
}
