//! Vim mode indicator rendering

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use crate::ui::vim::VimMode;

use super::ChatInput;

impl ChatInput {
    /// Get vim mode label and color if vim is active
    pub(super) fn get_vim_mode_label(&self, theme: &Theme, cx: &mut Context<Self>) -> Option<(&'static str, Hsla)> {
        self.vim_mode(cx).map(|m| match m {
            VimMode::Normal => ("NORMAL", theme.colors.accent),
            VimMode::Insert => ("INSERT", theme.colors.success),
            VimMode::Visual => ("VISUAL", theme.colors.warning),
            VimMode::VisualLine => ("V-LINE", theme.colors.warning),
            VimMode::VisualBlock => ("V-BLOCK", theme.colors.warning),
            VimMode::Command => ("CMD", theme.colors.info),
            VimMode::Search => ("SEARCH", theme.colors.info),
        })
    }

    /// Render vim mode indicator
    pub(super) fn render_vim_indicator(&self, label: &'static str, color: Hsla, theme: &Theme) -> impl IntoElement {
        let hint = match label {
            "NORMAL" => "Press 'i' to edit",
            "INSERT" => "Esc to return",
            _ => "",
        };

        div()
            .px_4()
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .px_2()
                    .py_px()
                    .bg(color)
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.colors.background)
                    .rounded_sm()
                    .child(label),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(hint),
            )
    }
}
