//! Shortcuts panel rendering

use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;
use super::state::ShortcutsPanel;
use super::types::*;

impl Render for ShortcutsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let filtered = self.filtered_groups();
        let search_query = self.search_query.clone();

        // Extract listeners before div chains
        let on_overlay_click = cx.listener(|_this, _, _window, cx| {
            cx.emit(ShortcutsPanelEvent::Dismissed);
        });

        let on_key_down = cx.listener(Self::handle_key_down);

        let on_close_click = cx.listener(|_this, _, _window, cx| {
            cx.emit(ShortcutsPanelEvent::Dismissed);
        });

        // Copy theme colors for move closures
        let surface_hover = theme.colors.surface_hover;
        let text_color = theme.colors.text;
        let accent_opacity = theme.colors.accent.opacity(0.5);
        let border_opacity = theme.colors.border.opacity(0.5);

        // Overlay background
        div()
            .id("shortcuts-panel-overlay")
            .absolute()
            .inset_0()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .flex()
            .items_center()
            .justify_center()
            .on_click(on_overlay_click)
            .child(
                // Panel modal
                div()
                    .id("shortcuts-panel")
                    .track_focus(&self.focus_handle)
                    .w(px(700.0))
                    .max_h(pct(85.0))
                    .bg(theme.colors.surface)
                    .rounded_xl()
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_xl()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_key_down(on_key_down)
                    .on_mouse_down(MouseButton::Left, |_, _window, _cx| {})
                    // Header
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_5()
                            .py_4()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child("Keyboard Shortcuts"),
                                    )
                                    .child(
                                        div()
                                            .px_2()
                                            .py_0p5()
                                            .rounded_md()
                                            .bg(theme.colors.accent.opacity(0.2))
                                            .text_xs()
                                            .text_color(theme.colors.accent)
                                            .child("Press ? to toggle"),
                                    ),
                            )
                            .child(
                                div()
                                    .id("shortcuts-close")
                                    .size(px(28.0))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(theme.colors.text_muted)
                                    .hover(move |s| s.bg(surface_hover).text_color(text_color))
                                    .cursor_pointer()
                                    .on_click(on_close_click)
                                    .child("x"),
                            ),
                    )
                    // Search bar
                    .child(
                        div()
                            .px_5()
                            .py_3()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .child(
                                div()
                                    .w_full()
                                    .h(px(36.0))
                                    .px_3()
                                    .rounded_lg()
                                    .bg(theme.colors.background)
                                    .border_1()
                                    .border_color(theme.colors.border)
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.colors.text_muted)
                                            .child("Search:"),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .text_sm()
                                            .text_color(theme.colors.text)
                                            .child(if search_query.is_empty() {
                                                "Type to filter shortcuts...".to_string()
                                            } else {
                                                search_query
                                            }),
                                    ),
                            ),
                    )
                    // Shortcuts list
                    .child(
                        div()
                            .flex_1()
                            .id("scroll-shortcuts")
                            .overflow_y_scroll()
                            .p_4()
                            .children(filtered.iter().map(|(group, shortcuts)| {
                                div()
                                    .mb_4()
                                    // Group header
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .mb_2()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(theme.colors.accent)
                                                    .child(group.name),
                                            )
                                            .child(
                                                div()
                                                    .h(px(1.0))
                                                    .flex_1()
                                                    .bg(theme.colors.border.opacity(0.5)),
                                            ),
                                    )
                                    // Shortcuts in group
                                    .child(
                                        div()
                                            .flex()
                                            .flex_wrap()
                                            .gap_2()
                                            .children(shortcuts.iter().map(|shortcut| {
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_3()
                                                    .px_3()
                                                    .py_2()
                                                    .rounded_lg()
                                                    .bg(theme.colors.background)
                                                    .border_1()
                                                    .border_color(border_opacity)
                                                    .hover(move |s| s.border_color(accent_opacity))
                                                    .min_w(px(200.0))
                                                    .child(
                                                        // Key combination
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap_1()
                                                            .children(
                                                                shortcut.keys.split('+').map(|key| {
                                                                    div()
                                                                        .px_1p5()
                                                                        .py_0p5()
                                                                        .rounded_sm()
                                                                        .bg(theme.colors.surface)
                                                                        .border_1()
                                                                        .border_color(theme.colors.border)
                                                                        .text_xs()
                                                                        .font_weight(FontWeight::MEDIUM)
                                                                        .text_color(theme.colors.text)
                                                                        .child(key.to_string())
                                                                })
                                                            ),
                                                    )
                                                    .child(
                                                        // Description
                                                        div()
                                                            .text_xs()
                                                            .text_color(theme.colors.text_muted)
                                                            .child(shortcut.description),
                                                    )
                                            })),
                                    )
                            })),
                    )
                    // Footer
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_5()
                            .py_3()
                            .border_t_1()
                            .border_color(theme.colors.border)
                            .bg(theme.colors.background.opacity(0.5))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_4()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(
                                                div()
                                                    .px_1()
                                                    .py_0p5()
                                                    .rounded_sm()
                                                    .bg(theme.colors.surface)
                                                    .border_1()
                                                    .border_color(theme.colors.border)
                                                    .child("Esc"),
                                            )
                                            .child("to close"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child("Type to search"),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child(format!(
                                        "{} shortcuts",
                                        SHORTCUT_GROUPS.iter().map(|g| g.shortcuts.len()).sum::<usize>()
                                    )),
                            ),
                    ),
            )
    }
}
