//! Individual toggle button renderers

use super::theme_colors::ThemeColors;
use gpui::*;

/// Configuration for a toggle button
pub struct ToggleButtonConfig {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
    pub is_active: Option<bool>,
    pub active_color: Hsla,
}

/// Render a generic toggle button with common styling
pub fn render_toggle_button(
    config: ToggleButtonConfig,
    colors: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let text_muted = colors.text_muted;
    let surface_hover = colors.surface_hover;
    let text = colors.text;

    let text_color = match config.is_active {
        Some(true) => config.active_color,
        _ => text_muted,
    };

    let mut button = div()
        .id(config.id)
        .flex()
        .items_center()
        .gap_1()
        .px_2()
        .py_1()
        .rounded_md()
        .cursor_pointer()
        .text_xs()
        .text_color(text_color)
        .hover(move |s| s.bg(surface_hover).text_color(text))
        .on_click(on_click)
        .child(config.label);

    if let Some(shortcut) = config.shortcut {
        button = button.child(
            div()
                .text_color(text_muted.opacity(0.5))
                .ml_1()
                .child(shortcut),
        );
    }

    button
}

/// Render timestamp toggle button
pub fn render_timestamp_toggle(
    show_timestamps: bool,
    colors: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    render_toggle_button(
        ToggleButtonConfig {
            id: "toggle-timestamps",
            label: "Time",
            shortcut: None,
            is_active: Some(show_timestamps),
            active_color: colors.accent,
        },
        colors,
        on_click,
    )
}

/// Render compact mode toggle button
pub fn render_compact_toggle(
    compact_mode: bool,
    colors: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    render_toggle_button(
        ToggleButtonConfig {
            id: "toggle-compact",
            label: "Compact",
            shortcut: None,
            is_active: Some(compact_mode),
            active_color: colors.accent,
        },
        colors,
        on_click,
    )
}

/// Render auto-scroll toggle button
pub fn render_auto_scroll_toggle(
    auto_scroll: bool,
    colors: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    render_toggle_button(
        ToggleButtonConfig {
            id: "toggle-auto-scroll",
            label: "Auto↓",
            shortcut: None,
            is_active: Some(auto_scroll),
            active_color: colors.accent,
        },
        colors,
        on_click,
    )
}

/// Render word wrap toggle button
pub fn render_word_wrap_toggle(
    word_wrap: bool,
    colors: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    render_toggle_button(
        ToggleButtonConfig {
            id: "toggle-word-wrap",
            label: "Wrap",
            shortcut: Some("⌥W"),
            is_active: Some(word_wrap),
            active_color: colors.accent,
        },
        colors,
        on_click,
    )
}

/// Render line numbers toggle button
pub fn render_line_numbers_toggle(
    show_line_numbers: bool,
    colors: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    render_toggle_button(
        ToggleButtonConfig {
            id: "toggle-line-numbers",
            label: "#",
            shortcut: Some("⌥L"),
            is_active: Some(show_line_numbers),
            active_color: colors.accent,
        },
        colors,
        on_click,
    )
}

/// Render stats toggle button
pub fn render_stats_toggle(
    show_stats_bar: bool,
    colors: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    render_toggle_button(
        ToggleButtonConfig {
            id: "toggle-stats",
            label: "Stats",
            shortcut: Some("⌘I"),
            is_active: Some(show_stats_bar),
            active_color: colors.accent,
        },
        colors,
        on_click,
    )
}

/// Render vim mode toggle button
pub fn render_vim_toggle(
    vim_enabled: bool,
    colors: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    render_toggle_button(
        ToggleButtonConfig {
            id: "toggle-vim-mode",
            label: "Vim",
            shortcut: Some("⌃⇧V"),
            is_active: Some(vim_enabled),
            active_color: colors.success,
        },
        colors,
        on_click,
    )
}

/// Render theme toggle button
pub fn render_theme_toggle(
    is_dark: bool,
    colors: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    render_toggle_button(
        ToggleButtonConfig {
            id: "toggle-theme",
            label: if is_dark { "☀" } else { "🌙" },
            shortcut: Some("⇧⌘T"),
            is_active: None,
            active_color: colors.text_muted,
        },
        colors,
        on_click,
    )
}

/// Render search toggle button
pub fn render_search_toggle(
    show_search: bool,
    colors: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    render_toggle_button(
        ToggleButtonConfig {
            id: "toggle-search",
            label: "Search",
            shortcut: Some("⌘F"),
            is_active: Some(show_search),
            active_color: colors.accent,
        },
        colors,
        on_click,
    )
}

/// Render command palette toggle button (special hover style)
pub fn render_command_palette_toggle(
    colors: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let accent = colors.accent;
    let text_muted = colors.text_muted;

    div()
        .id("toggle-command-palette")
        .flex()
        .items_center()
        .gap_1()
        .px_2()
        .py_1()
        .rounded_md()
        .cursor_pointer()
        .text_xs()
        .text_color(text_muted)
        .hover(move |s| s.bg(accent.opacity(0.1)).text_color(accent))
        .on_click(on_click)
        .child("⌘")
        .child(div().text_color(text_muted.opacity(0.5)).ml_1().child("⌘K"))
}

/// Render shortcuts help toggle button
pub fn render_shortcuts_toggle(
    colors: &ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    render_toggle_button(
        ToggleButtonConfig {
            id: "toggle-shortcuts-help",
            label: "?",
            shortcut: Some("⌘?"),
            is_active: None,
            active_color: colors.text_muted,
        },
        colors,
        on_click,
    )
}
