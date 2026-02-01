//! Highlight functionality for code blocks

use gpui::*;

use super::super::types::{HighlightStyle, HighlightedRange};
use super::CodeBlockView;

impl CodeBlockView {
    /// Add a highlighted range
    pub fn add_highlight(&mut self, range: HighlightedRange, cx: &mut Context<Self>) {
        self.highlighted_ranges.push(range);
        cx.notify();
    }

    /// Highlight a single line
    pub fn highlight_line(&mut self, line: usize, style: HighlightStyle, cx: &mut Context<Self>) {
        self.highlighted_ranges
            .push(HighlightedRange::single(line, style));
        cx.notify();
    }

    /// Highlight a range of lines
    pub fn highlight_range(
        &mut self,
        start: usize,
        end: usize,
        style: HighlightStyle,
        cx: &mut Context<Self>,
    ) {
        self.highlighted_ranges
            .push(HighlightedRange::range(start, end, style));
        cx.notify();
    }

    /// Clear all highlights
    pub fn clear_highlights(&mut self, cx: &mut Context<Self>) {
        self.highlighted_ranges.clear();
        cx.notify();
    }

    /// Get the highlight style for a line (1-based), if any
    pub(crate) fn get_line_highlight(&self, line: usize) -> Option<HighlightStyle> {
        // Return the first matching highlight (could prioritize differently)
        for range in &self.highlighted_ranges {
            if range.contains(line) {
                return Some(range.style);
            }
        }
        None
    }

    /// Check if any lines are highlighted
    pub fn has_highlights(&self) -> bool {
        !self.highlighted_ranges.is_empty()
    }

    /// Get background color for a highlight style
    pub(crate) fn highlight_bg_color(
        &self,
        style: HighlightStyle,
        theme: &crate::app::theme::Theme,
    ) -> Hsla {
        use HighlightStyle as HS;
        match style {
            HS::Reference => theme.colors.accent.opacity(0.15),
            HS::Error => theme.colors.error.opacity(0.15),
            HS::Warning => theme.colors.warning.opacity(0.15),
            HS::Success => theme.colors.success.opacity(0.15),
            HS::Info => theme.colors.info.opacity(0.15),
            HS::Emphasis => hsla(280.0 / 360.0, 0.6, 0.5, 0.15), // Purple
        }
    }

    /// Get border color for a highlight style (for gutter indicator)
    pub(crate) fn highlight_border_color(
        &self,
        style: HighlightStyle,
        theme: &crate::app::theme::Theme,
    ) -> Hsla {
        use HighlightStyle as HS;
        match style {
            HS::Reference => theme.colors.accent,
            HS::Error => theme.colors.error,
            HS::Warning => theme.colors.warning,
            HS::Success => theme.colors.success,
            HS::Info => theme.colors.info,
            HS::Emphasis => hsla(280.0 / 360.0, 0.6, 0.5, 1.0), // Purple
        }
    }
}
