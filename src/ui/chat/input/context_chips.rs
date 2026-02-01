//! Context chips rendering for file mentions

use gpui::prelude::*;
use gpui::*;

use super::utils::get_file_icon;
use super::ChatInput;
use crate::app::theme::Theme;

impl ChatInput {
    /// Render attached file badges/chips with remove buttons
    pub(super) fn render_context_chips(
        &self,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let file_mentions = self.file_mentions();
        let file_count = file_mentions.len();

        div()
            .px_4()
            .flex()
            .flex_wrap()
            .items_center()
            .gap_2()
            .children(file_mentions.iter().take(5).enumerate().map(|(idx, path)| {
                let path_clone = path.clone();
                let filename = path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.to_string_lossy().to_string());

                // Get file icon based on extension
                let icon = get_file_icon(&filename);

                self.render_file_chip(idx, icon, filename, path_clone, theme, cx)
            }))
            .when(file_count > 5, |d| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .child(format!("+{} more", file_count - 5)),
                )
            })
    }

    /// Render a single file chip with remove button
    fn render_file_chip(
        &self,
        idx: usize,
        icon: &'static str,
        filename: String,
        path: std::path::PathBuf,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let path_for_remove = path.clone();

        div()
            .id(ElementId::Name(format!("file-chip-{}", idx).into()))
            .px_2()
            .py(px(2.0))
            .bg(theme.colors.accent.opacity(0.15))
            .text_color(theme.colors.accent)
            .text_xs()
            .rounded_sm()
            .flex()
            .items_center()
            .gap_1()
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.accent.opacity(0.25)))
            // File icon
            .child(icon)
            // Filename
            .child(filename)
            // Remove button (X)
            .child(
                div()
                    .id(ElementId::Name(format!("remove-file-{}", idx).into()))
                    .ml_1()
                    .size(px(14.0))
                    .rounded_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(theme.colors.text_muted)
                    .hover(|s| {
                        s.bg(theme.colors.error.opacity(0.3))
                            .text_color(theme.colors.error)
                    })
                    .on_click(cx.listener(move |this, _event, _window, cx| {
                        this.remove_file_mention(&path_for_remove, cx);
                    }))
                    .child("✕"),
            )
    }
}
