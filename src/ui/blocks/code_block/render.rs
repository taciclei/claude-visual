//! Render implementation for CodeBlockView

use gpui::*;
use gpui::prelude::*;

use super::view::CodeBlockView;
use super::types::CodeDisplayMode;

impl Render for CodeBlockView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let collapsed = self.collapsed;
        let search_visible = self.search_visible;
        let display_mode = self.display_mode;

        div()
            .w_full()
            .rounded_lg()
            .overflow_hidden()
            .border_1()
            .border_color(theme.colors.border)
            // Header
            .child(self.render_header(cx))
            // Search bar (when visible)
            .when(search_visible, |this| {
                this.child(self.render_search_bar(cx))
            })
            // Code content
            .when(!collapsed, |this| {
                if display_mode == CodeDisplayMode::Diff && !self.diff_lines.is_empty() {
                    // Diff view rendering
                    this.child(self.render_diff_view(cx))
                } else {
                    // Normal code view
                    this.child(self.render_code_view(cx))
                }
            })
    }
}
