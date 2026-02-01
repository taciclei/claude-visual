//! Templates panel render functions

mod header;
mod search;
mod list;
mod footer;

use gpui::*;
use gpui::prelude::*;

use super::super::core::ChatView;

impl ChatView {
    pub fn render_templates_panel(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let templates_by_cat = self.templates_by_category();
        let filter = self.templates_filter.clone();
        let total_count = self.prompt_templates.len();
        let custom_count = self.prompt_templates.iter().filter(|t| !t.is_builtin).count();

        let overlay_listener = cx.listener(|this, _, _window, cx| {
            this.toggle_templates_panel(cx);
        });

        let surface_color = theme.colors.surface;
        let border_color = theme.colors.border;

        div()
            .id("templates-panel-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(overlay_listener)
            .child(
                div()
                    .id("templates-panel")
                    .w(px(600.0))
                    .max_h(px(500.0))
                    .bg(surface_color)
                    .rounded_lg()
                    .border_1()
                    .border_color(border_color)
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_click(|_, _, _| {})
                    .child(self.render_templates_header(theme, total_count, custom_count, cx))
                    .child(self.render_templates_search(theme, filter, cx))
                    .child(self.render_templates_list(theme, templates_by_cat, cx))
                    .child(self.render_templates_footer(theme, cx))
            )
    }
}
