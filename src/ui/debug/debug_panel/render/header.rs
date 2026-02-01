//! Debug panel header and main render implementation

use gpui::*;
use gpui::prelude::*;

use crate::ui::debug::debug_panel::DebugPanel;

impl Render for DebugPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let expanded = self.expanded;
        let show_ai_menu = self.show_ai_menu;
        let current_location = self.current_location.clone();
        let active_tab = self.active_tab;

        // Extract listener before div chain
        let on_header_click = cx.listener(|this, _, _window, cx| {
            this.toggle_expanded(cx);
        });

        // Copy theme colors for move closures
        let background_color = theme.colors.background;
        let border_color = theme.colors.border;
        let surface_color = theme.colors.surface;
        let text_muted_color = theme.colors.text_muted;
        let text_color = theme.colors.text;

        div()
            .w_full()
            .flex()
            .flex_col()
            .bg(background_color)
            .border_t_1()
            .border_color(border_color)
            // Header
            .child(
                div()
                    .id("debug-panel-header")
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_2()
                    .py_1()
                    .bg(surface_color)
                    .cursor_pointer()
                    .on_click(on_header_click)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(text_muted_color)
                                    .child(if expanded { "▼" } else { "▶" }),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(text_color)
                                    .child("Debug"),
                            ),
                    )
                    // Location
                    .when(current_location.is_some(), |d| {
                        let (file, line) = current_location.as_ref().unwrap();
                        d.child(
                            div()
                                .text_xs()
                                .text_color(text_muted_color)
                                .child(format!("{}:{}", file, line)),
                        )
                    }),
            )
            // Content
            .when(expanded, |d| {
                d.child(
                    div()
                        .relative()
                        .child(self.render_toolbar(&theme, cx))
                        // AI menu overlay
                        .when(show_ai_menu, |d| {
                            d.child(self.render_ai_menu(&theme, cx))
                        }),
                )
                    .child(self.render_tabs(&theme, cx))
                    .child(
                        div()
                            .h(px(200.0))
                            .flex()
                            .flex_col()
                            .child(match active_tab {
                                crate::ui::debug::debug_panel::types::DebugTab::Console => self.render_console(&theme).into_any_element(),
                                _ => div()
                                    .flex_1()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_sm()
                                    .text_color(text_muted_color)
                                    .child(format!("{} view", active_tab.label()))
                                    .into_any_element(),
                            }),
                    )
            })
    }
}
