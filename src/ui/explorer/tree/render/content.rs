//! Content area rendering (loading, error, file list)

use gpui::*;
use gpui::prelude::*;

use crate::ui::explorer::tree::core::FileTree;
use crate::ui::explorer::tree::types::SimpleColors;

impl FileTree {
    pub(crate) fn render_content(&self, theme: &SimpleColors, cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_loading {
            return div()
                .flex()
                .items_center()
                .justify_center()
                .py_8()
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.text_muted)
                        .child("Loading..."),
                )
                .into_any_element();
        }

        if let Some(error) = &self.error {
            return div()
                .flex()
                .items_center()
                .justify_center()
                .py_8()
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.error)
                        .child(error.clone()),
                )
                .into_any_element();
        }

        if let Some(ref root) = self.root {
            div()
                .flex()
                .flex_col()
                .children(
                    root.visible_entries()
                        .iter()
                        .filter(|e| self.matches_filter(e))
                        .map(|entry| self.render_entry(entry, &theme, cx)),
                )
                .into_any_element()
        } else {
            div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .py_8()
                .gap_2()
                .child(
                    div()
                        .text_2xl()
                        .text_color(theme.text_muted)
                        .child("📂"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.text_muted)
                        .child("No folder open"),
                )
                .into_any_element()
        }
    }
}
