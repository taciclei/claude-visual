//! Export panel render functions

mod actions;
mod format_selector;
mod header;
mod options;
mod stats;

use gpui::prelude::*;
use gpui::*;

use crate::ui::chat::view::core::ChatView;

impl ChatView {
    pub(crate) fn render_export_panel(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let on_overlay_click = cx.listener(|this, _, _window, cx| {
            this.toggle_export_panel(cx);
        });

        div()
            .id("export-panel-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(on_overlay_click)
            .child(
                div()
                    .id("export-panel")
                    .w(px(450.0))
                    .max_h(px(500.0))
                    .bg(theme.colors.surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_click(|_, _, _| {})
                    // Header
                    .child(self.render_export_header(theme, cx))
                    // Stats summary
                    .child(self.render_export_stats(theme))
                    // Format selection
                    .child(self.render_export_format_selector(theme, cx))
                    // Options
                    .child(self.render_export_options(theme, cx))
                    // Actions
                    .child(self.render_export_actions(theme, cx)),
            )
    }
}
