//! Watch view rendering

mod header;
mod add_input;
mod expression_list;
mod empty_state;

use gpui::*;
use gpui::prelude::*;

use super::core::WatchView;

impl Render for WatchView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let is_adding = self.is_adding;

        div()
            .w_full()
            .flex()
            .flex_col()
            // Header
            .child(self.render_header(&theme, cx))
            // Add expression input
            .when(is_adding, |d| {
                d.child(self.render_add_input(&theme, cx))
            })
            // Expression list
            .child(self.render_expression_list(&theme, cx))
            // Empty state
            .when(self.expressions.is_empty() && !is_adding, |d| {
                d.child(self.render_empty_state(&theme, cx))
            })
    }
}
