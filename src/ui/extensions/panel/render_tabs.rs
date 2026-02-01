use gpui::*;
use gpui::prelude::*;

use super::types::*;
use super::panel::ExtensionsPanel;

impl Render for ExtensionsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        // Backdrop
        div()
            .id("extensions-panel-backdrop")
            .track_focus(&self.focus_handle)
            .absolute()
            .inset_0()
            .bg(theme.colors.background.opacity(0.8))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                this.dismiss(cx);
            }))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                if event.keystroke.key == "escape" {
                    this.dismiss(cx);
                }
            }))
            // Panel container
            .child(
                div()
                    .id("extensions-panel")
                    .w(px(800.0))
                    .h(px(600.0))
                    .rounded_xl()
                    .bg(theme.colors.background)
                    .border_1()
                    .border_color(theme.colors.border)
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_mouse_down(MouseButton::Left, |_, _window, cx| {
                    })
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
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.colors.text)
                                    .child("Extensions"),
                            )
                            .child(
                                div()
                                    .id("extensions-close")
                                    .size(px(28.0))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| {
                                        s.bg(theme.colors.surface_hover)
                                            .text_color(theme.colors.text)
                                    })
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.dismiss(cx);
                                    }))
                                    .child("x"),
                            ),
                    )
                    // Tabs
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .px_4()
                            .py_2()
                            .gap_4()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .children(ExtensionsTab::all().iter().map(|&tab| {
                                let is_active = self.active_tab == tab;
                                div()
                                    .id(SharedString::from(format!("ext-tab-{}", tab.label())))
                                    .px_1()
                                    .py_1()
                                    .cursor_pointer()
                                    .text_sm()
                                    .when(is_active, |d| {
                                        d.text_color(theme.colors.accent)
                                            .border_b_2()
                                            .border_color(theme.colors.accent)
                                    })
                                    .when(!is_active, |d| {
                                        d.text_color(theme.colors.text_muted)
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
                            .overflow_hidden()
                            .when(self.active_tab == ExtensionsTab::Installed, |d| {
                                d.child(self.render_installed_tab(cx))
                            })
                            .when(self.active_tab == ExtensionsTab::Available, |d| {
                                d.child(self.render_available_tab(cx))
                            })
                            .when(self.active_tab == ExtensionsTab::Updates, |d| {
                                d.child(self.render_updates_tab(cx))
                            }),
                    ),
            )
    }
}
