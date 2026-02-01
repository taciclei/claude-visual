//! View options render functions

mod context_usage;
mod theme_colors;
mod toggle_buttons;

use gpui::*;
use gpui::prelude::*;
use crate::ui::chat::view::core::ChatView;
use theme_colors::ThemeColors;
use toggle_buttons::*;

impl ChatView {
    /// Renders all view option toggles
    pub fn render_view_options(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        // Extract state
        let show_timestamps = self.show_timestamps;
        let compact_mode = self.compact_mode;
        let auto_scroll = self.auto_scroll;
        let word_wrap = self.word_wrap;
        let show_line_numbers = self.show_line_numbers;
        let show_stats_bar = self.show_stats;
        let vim_enabled = self.is_vim_mode_enabled(cx);
        let is_dark = theme.is_dark;
        let show_search = self.show_search;

        // Copy theme colors for move closures
        let colors = ThemeColors::from_theme(theme);

        // Extract all listeners before div chains
        let on_timestamp_click = cx.listener(|this, _, _window, cx| {
            this.toggle_timestamps(cx);
        });
        let on_compact_click = cx.listener(|this, _, _window, cx| {
            this.toggle_compact_mode(cx);
        });
        let on_auto_scroll_click = cx.listener(|this, _, _window, cx| {
            this.toggle_auto_scroll(cx);
        });
        let on_word_wrap_click = cx.listener(|this, _, _window, cx| {
            this.toggle_word_wrap(cx);
        });
        let on_line_numbers_click = cx.listener(|this, _, _window, cx| {
            this.toggle_line_numbers(cx);
        });
        let on_stats_click = cx.listener(|this, _, _window, cx| {
            this.toggle_stats(cx);
        });
        let on_vim_click = cx.listener(|this, _, _window, cx| {
            this.toggle_vim_mode(cx);
        });
        let on_theme_click = cx.listener(|this, _, _window, cx| {
            this.request_theme_toggle(cx);
        });
        let on_search_click = cx.listener(|this, _, _window, cx| {
            this.toggle_search(cx);
        });
        let on_command_palette_click = cx.listener(|this, _, _window, cx| {
            this.toggle_command_palette(cx);
        });
        let on_shortcuts_click = cx.listener(|this, _, _window, cx| {
            this.toggle_shortcuts_help(cx);
        });

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(render_timestamp_toggle(show_timestamps, &colors, on_timestamp_click))
            .child(render_compact_toggle(compact_mode, &colors, on_compact_click))
            .child(render_auto_scroll_toggle(auto_scroll, &colors, on_auto_scroll_click))
            .child(render_word_wrap_toggle(word_wrap, &colors, on_word_wrap_click))
            .child(render_line_numbers_toggle(show_line_numbers, &colors, on_line_numbers_click))
            .child(render_stats_toggle(show_stats_bar, &colors, on_stats_click))
            .child(render_vim_toggle(vim_enabled, &colors, on_vim_click))
            .child(render_theme_toggle(is_dark, &colors, on_theme_click))
            .child(render_search_toggle(show_search, &colors, on_search_click))
            .child(render_command_palette_toggle(&colors, on_command_palette_click))
            .child(render_shortcuts_toggle(&colors, on_shortcuts_click))
            .when(self.context_used > 0 || self.session_input_tokens > 0, |d| {
                d.child(self.render_context_usage_indicator(theme, cx))
            })
    }
}
