//! Main Render implementation for ThemeEditor

use gpui::*;
use gpui::prelude::*;

use crate::ui::extensions::theme_editor::ThemeEditor;
use crate::ui::extensions::theme_editor::types::ThemeEditorTab;

impl Render for ThemeEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.colors.background)
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.colors.text)
                                    .child(if self.is_new { "New Theme" } else { "Edit Theme" }),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .bg(theme.colors.surface)
                                    .border_1()
                                    .border_color(theme.colors.border)
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .child(self.theme_name.clone()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            // Reset button
                            .when(self.original_theme.is_some(), |d| {
                                d.child(
                                    div()
                                        .id("reset-theme")
                                        .px_3()
                                        .py_1()
                                        .rounded_md()
                                        .cursor_pointer()
                                        .text_sm()
                                        .text_color(theme.colors.text_muted)
                                        .hover(|s| s.bg(theme.colors.surface_hover))
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.reset(cx);
                                        }))
                                        .child("Reset"),
                                )
                            })
                            // Save button
                            .child(
                                div()
                                    .id("save-theme")
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .when(self.has_changes, |d| {
                                        d.bg(theme.colors.accent)
                                            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                    })
                                    .when(!self.has_changes, |d| {
                                        d.bg(theme.colors.border)
                                            .text_color(theme.colors.text_muted)
                                    })
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        if this.has_changes {
                                            this.save(cx);
                                        }
                                    }))
                                    .child("Save"),
                            )
                            // Close button
                            .child(
                                div()
                                    .id("close-editor")
                                    .size(px(28.0))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.close(cx);
                                    }))
                                    .child("x"),
                            ),
                    ),
            )
            // Tabs
            .child(
                div()
                    .flex()
                    .px_4()
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .children(ThemeEditorTab::all().iter().map(|&tab| {
                        let is_active = self.active_tab == tab;
                        div()
                            .id(SharedString::from(format!("tab-{}", tab.label())))
                            .px_3()
                            .py_2()
                            .cursor_pointer()
                            .text_sm()
                            .border_b_2()
                            .when(is_active, |d| {
                                d.border_color(theme.colors.accent)
                                    .text_color(theme.colors.accent)
                                    .font_weight(FontWeight::MEDIUM)
                            })
                            .when(!is_active, |d| {
                                d.border_color(gpui::transparent_black())
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.text_color(theme.colors.text))
                            })
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.switch_tab(tab, cx);
                            }))
                            .child(tab.label())
                    })),
            )
            // Content
            .child(
                div()
                    .flex_1()
                    .flex()
                    .overflow_hidden()
                    // Main content
                    .child(
                        div()
                            .flex_1()
                            .p_4()
                            .id("scroll-theme-editor-content")
                            .overflow_y_scroll()
                            .when(self.active_tab == ThemeEditorTab::Colors, |d| {
                                d.child(self.render_colors_tab(cx))
                            })
                            .when(self.active_tab == ThemeEditorTab::Syntax, |d| {
                                d.child(self.render_syntax_tab(cx))
                            })
                            .when(self.active_tab == ThemeEditorTab::Preview, |d| {
                                d.child(self.render_preview_tab(cx))
                            })
                            .when(self.active_tab == ThemeEditorTab::Export, |d| {
                                d.child(self.render_export_tab(cx))
                            }),
                    )
                    // Color picker sidebar (when color is selected)
                    .when(self.selected_color.is_some(), |d| {
                        d.child(
                            div()
                                .w(px(250.0))
                                .p_3()
                                .border_l_1()
                                .border_color(theme.colors.border)
                                .child(self.render_color_picker(
                                    self.selected_color.unwrap(),
                                    theme,
                                    cx,
                                )),
                        )
                    }),
            )
    }
}
