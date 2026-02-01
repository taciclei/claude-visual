//! Context usage indicator rendering

use crate::ui::chat::view::core::ChatView;
use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    /// Renders the context usage indicator with progress bar
    pub fn render_context_usage_indicator(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let usage_pct = self.context_usage_percentage();
        let usage_color = if usage_pct < 0.5 {
            theme.colors.success
        } else if usage_pct < 0.8 {
            theme.colors.warning
        } else {
            theme.colors.error
        };

        // Copy colors for move closures
        let usage_color_hover = usage_color;
        let surface_bg = theme.colors.surface;

        // Extract listener before div chain
        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_session_details(cx);
        });

        div()
            .flex()
            .items_center()
            .gap_0()
            .child(div().w(px(1.0)).h(px(16.0)).bg(theme.colors.border).mx_1())
            .child(
                div()
                    .id("context-usage-mini")
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(usage_color.opacity(0.1))
                    .hover(move |s| s.bg(usage_color_hover.opacity(0.15)))
                    .on_click(on_click)
                    .child(
                        div()
                            .text_xs()
                            .text_color(usage_color)
                            .child(format!("{:.0}%", usage_pct * 100.0)),
                    )
                    .child(
                        div()
                            .w(px(30.0))
                            .h(px(4.0))
                            .rounded_sm()
                            .bg(surface_bg)
                            .child(
                                div()
                                    .h_full()
                                    .rounded_sm()
                                    .bg(usage_color)
                                    .w(pct((usage_pct * 100.0) as f32)),
                            ),
                    ),
            )
    }
}
