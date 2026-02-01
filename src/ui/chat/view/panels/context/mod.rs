//! Context panel render functions

mod header;
mod usage_bar;
mod files_section;
mod tools_section;
mod session_info;
mod footer;

use gpui::*;
use gpui::prelude::*;

use super::super::core::ChatView;

impl ChatView {
    pub fn render_context_panel(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_clone = theme.clone();
        let session_info = self.session_info.as_ref();

        div()
            .id("context-panel-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_context_panel(cx);
            }))
            .child(
                div()
                    .id("context-panel")
                    .w(px(550.0))
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
                    .child(self.render_context_header(&theme_clone, cx))
                    // Context usage bar
                    .child(self.render_usage_bar(&theme_clone))
                    // Content scrollable area
                    .child(
                        div()
                            .id("context-panel-content")
                            .flex_1()
                            .overflow_y_scroll()
                            .p_4()
                            // Files section
                            .child(self.render_files_section(&theme_clone, cx))
                            // Tools section
                            .when_some(session_info, |d, info| {
                                d.child(self.render_tools_section(&theme_clone, info))
                            })
                            // Session info
                            .when_some(session_info, |d, info| {
                                d.child(self.render_session_info(&theme_clone, info))
                            })
                    )
                    // Footer
                    .child(self.render_context_footer(&theme_clone, cx))
            )
    }
}
