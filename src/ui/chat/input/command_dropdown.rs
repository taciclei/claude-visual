//! Slash command autocomplete dropdown rendering

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;

use super::ChatInput;
use super::utils::{get_command_description, get_command_icon};

/// Get the category for a command
fn get_command_category(cmd: &str) -> (&'static str, &'static str) {
    let sl = cmd.to_lowercase();

    // Implementation
    if sl == "apex" || sl == "oneshot" || sl == "ultrathink" || sl == "plan" {
        return ("⚡", "Implementation");
    }
    // Exploration
    if sl == "explore" || sl == "search" || sl == "explain" || sl == "docs" {
        return ("🔍", "Exploration");
    }
    // Code Quality
    if sl.contains("review") || sl == "refactor" || sl == "clean-code" || sl == "debug" || sl == "add-llm-comments" {
        return ("✨", "Code Quality");
    }
    // Research
    if sl == "brainstorm" {
        return ("💡", "Research");
    }
    // Git & CI
    if sl == "commit" || sl.contains("pr") || sl == "merge" || sl.contains("ci") || sl == "fix-pr-comments" {
        return ("📦", "Git & CI");
    }
    // Session
    if sl == "usage" || sl == "compact" || sl == "memory" || sl == "resume" || sl == "clear" || sl == "cost" || sl == "status" {
        return ("📊", "Session");
    }
    // Config
    if sl == "config" || sl == "permissions" || sl == "model" || sl == "vim" || sl == "hooks" || sl == "allowed-tools" || sl == "mcp" {
        return ("⚙️", "Configuration");
    }
    // Skill Creation
    if sl.contains("create-") || sl == "keybindings-help" || sl == "claude-memory" {
        return ("🛠️", "Skill Creation");
    }
    // Utilities
    if sl.contains("auto-") || sl == "watch-ci" || sl == "fix-grammar" || sl == "debug-ccli" {
        return ("🔧", "Utilities");
    }
    // Help
    if sl == "help" || sl == "doctor" || sl == "bug" {
        return ("❓", "Help");
    }
    ("⚙️", "Other")
}

impl ChatInput {
    /// Render slash command autocomplete dropdown with fuzzy match highlighting
    pub(super) fn render_command_dropdown(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_idx = self.selected_command_index;
        let accent = theme.colors.accent;
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;

        // Check if we should show quick access chips (only when not filtering much)
        let show_categories = self.text.len() <= 2; // Just "/" or "/x"

        div()
            .id("command-autocomplete")
            .absolute()
            .bottom(px(80.0))
            .left(px(16.0))
            .right(px(16.0))
            .max_h(px(320.0))
            .overflow_y_scroll()
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.border)
            .rounded_lg()
            .shadow_lg()
            .p_1()
            .flex()
            .flex_col()
            .gap_px()
            // Quick skill chips at top
            .when(show_categories, |d| {
                d.child(
                    div()
                        .px_2()
                        .py_1()
                        .mb_1()
                        .flex()
                        .flex_wrap()
                        .gap_1()
                        .children(
                            [
                                ("⚡", "apex", "APEX workflow"),
                                ("🔍", "explore", "Explore code"),
                                ("🐛", "debug", "Debug errors"),
                                ("📦", "commit", "Git commit"),
                                ("👀", "review", "Code review"),
                                ("🚀", "oneshot", "Quick implement"),
                            ].into_iter().map(|(icon, cmd, _desc)| {
                                let cmd_str = cmd.to_string();
                                let is_hot = cmd == "apex" || cmd == "debug";
                                div()
                                    .id(SharedString::from(format!("quick-{}", cmd)))
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py(px(3.0))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_xs()
                                    .when(is_hot, |d| {
                                        d.bg(accent.opacity(0.15))
                                            .border_1()
                                            .border_color(accent.opacity(0.3))
                                            .text_color(accent)
                                    })
                                    .when(!is_hot, |d| {
                                        d.bg(theme.colors.surface_hover)
                                            .text_color(text_muted)
                                    })
                                    .hover(|s| s.bg(accent.opacity(0.2)).text_color(accent))
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.text = format!("/{} ", cmd_str);
                                        this.cursor_position = this.text.len();
                                        this.show_command_autocomplete = false;
                                        this.filtered_commands.clear();
                                        this.command_matches.clear();
                                        cx.notify();
                                    }))
                                    .child(icon)
                                    .child(cmd)
                            })
                        )
                )
                .child(
                    div()
                        .h(px(1.0))
                        .mx_2()
                        .mb_1()
                        .bg(theme.colors.border.opacity(0.5))
                )
            })
            // Command list
            .children(
                self.command_matches.iter().enumerate().map(|(i, m)| {
                    let is_selected = i == selected_idx;
                    let cmd_clone = m.command.clone();
                    let matched_indices = m.matched_indices.clone();
                    let icon = get_command_icon(&m.command);
                    let (cat_icon, _cat_name) = get_command_category(&m.command);

                    div()
                        .id(SharedString::from(format!("cmd-{}", i)))
                        .px_3()
                        .py_2()
                        .rounded_md()
                        .cursor_pointer()
                        .bg(if is_selected { theme.colors.accent.opacity(0.15) } else { gpui::transparent_black() })
                        .when(is_selected, |d| d.border_l_2().border_color(accent))
                        .hover(|s| s.bg(theme.colors.surface_hover))
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            this.text = format!("/{} ", cmd_clone);
                            this.cursor_position = this.text.len();
                            this.show_command_autocomplete = false;
                            this.filtered_commands.clear();
                            this.command_matches.clear();
                            cx.notify();
                        }))
                        .flex()
                        .items_center()
                        .gap_3()
                        // Category icon (subtle)
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_muted.opacity(0.5))
                                .w(px(16.0))
                                .text_center()
                                .child(cat_icon)
                        )
                        // Skill icon
                        .child(
                            div()
                                .text_sm()
                                .child(icon)
                        )
                        // Command name with highlighted matches
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .text_sm()
                                .min_w(px(100.0))
                                .child(
                                    div()
                                        .text_color(accent)
                                        .child("/")
                                )
                                .children(
                                    render_highlighted_command(&m.command, &matched_indices, accent, text_color)
                                )
                        )
                        // Description
                        .child(
                            div()
                                .flex_1()
                                .text_xs()
                                .text_color(text_muted)
                                .truncate()
                                .child(get_command_description(&m.command))
                        )
                        // Selection indicator
                        .when(is_selected, |d| {
                            d.child(
                                div()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .child("↵")
                            )
                        })
                })
            )
            // Footer with shortcuts
            .child(
                div()
                    .mt_1()
                    .pt_1()
                    .border_t_1()
                    .border_color(theme.colors.border)
                    .px_2()
                    .py_1()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .text_xs()
                            .text_color(text_muted)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .px_1()
                                            .rounded_sm()
                                            .bg(theme.colors.surface_hover)
                                            .font_family("monospace")
                                            .child("↑↓")
                                    )
                                    .child("navigate")
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .px_1()
                                            .rounded_sm()
                                            .bg(theme.colors.surface_hover)
                                            .font_family("monospace")
                                            .child("Tab")
                                    )
                                    .child("select")
                            )
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(accent)
                            .child(format!("{} commands", self.command_matches.len()))
                    )
            )
    }
}

/// Render command name with matched characters highlighted
fn render_highlighted_command(cmd: &str, matched_indices: &[usize], accent: gpui::Hsla, text_color: gpui::Hsla) -> Vec<Div> {
    let chars: Vec<char> = cmd.chars().collect();
    let mut result = Vec::new();
    let mut current_text = String::new();
    let mut current_is_matched = false;

    for (i, c) in chars.iter().enumerate() {
        let is_matched = matched_indices.contains(&i);

        if i == 0 {
            current_is_matched = is_matched;
            current_text.push(*c);
        } else if is_matched == current_is_matched {
            current_text.push(*c);
        } else {
            // State changed, push current segment
            if !current_text.is_empty() {
                result.push(
                    div()
                        .text_color(if current_is_matched { accent } else { text_color })
                        .when(current_is_matched, |d| d.font_weight(FontWeight::BOLD))
                        .child(current_text.clone())
                );
            }
            current_text = String::new();
            current_text.push(*c);
            current_is_matched = is_matched;
        }
    }

    // Push final segment
    if !current_text.is_empty() {
        result.push(
            div()
                .text_color(if current_is_matched { accent } else { text_color })
                .when(current_is_matched, |d| d.font_weight(FontWeight::BOLD))
                .child(current_text)
        );
    }

    result
}
