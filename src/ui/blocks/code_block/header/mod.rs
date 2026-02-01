//! Code block header rendering

mod title;
mod actions;

use gpui::*;
use gpui::prelude::*;

use super::types::CodeDisplayMode;
use super::view::CodeBlockView;

impl CodeBlockView {
    /// Render the code block header
    pub(crate) fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let collapsed = self.collapsed;
        let is_executable = self.is_executable();
        let search_visible = self.search_visible;

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_2()
            .bg(theme.colors.surface)
            .border_b_1()
            .border_color(theme.colors.border)
            .child(title::render_title(self, cx))
            .child(self.render_header_buttons(collapsed, is_executable, search_visible, cx))
    }

    /// Render header action buttons
    fn render_header_buttons(
        &self,
        collapsed: bool,
        is_executable: bool,
        search_visible: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let show_copied = self.show_copied_feedback;

        div()
            .flex()
            .items_center()
            .gap_2()
            .when(is_executable, |d| {
                d.child(actions::render_execute_button(&theme, cx))
            })
            .child(actions::render_save_button(&theme, cx))
            .child(actions::render_copy_button(show_copied, &theme, cx))
            .child(actions::render_search_button(search_visible, &theme, cx))
            .when(self.has_diff(), |d| {
                d.child(actions::render_diff_button(self, &theme, cx))
            })
            .child(actions::render_explain_button(&theme, cx))
            .child(actions::render_improve_button(&theme, cx))
            .child(actions::render_test_button(&theme, cx))
            .child(actions::render_review_button(&theme, cx))
            .child(actions::render_refactor_button(&theme, cx))
            .child(actions::render_collapse_button(collapsed, &theme, cx))
    }
}
