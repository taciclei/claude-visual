use super::super::SettingsModal;
use gpui::prelude::*;
use gpui::*;

impl SettingsModal {
    /// Render the editor tab
    pub(crate) fn render_editor_tab(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .gap_4()
            // Code font settings
            .child(
                self.render_section(
                    "Code Font",
                    "Font used for code blocks and input",
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(self.render_font_select(
                            "Font family",
                            &self.pending.editor.code_font_family,
                            &["JetBrains Mono", "Fira Code", "SF Mono", "Menlo", "Monaco"],
                            |this, value, cx| {
                                this.pending.editor.code_font_family = value.to_string();
                                this.mark_changed(cx);
                            },
                            cx,
                        ))
                        .child(self.render_slider(
                            "Font size",
                            self.pending.editor.code_font_size,
                            10.0,
                            24.0,
                            |this, value, cx| {
                                this.pending.editor.code_font_size = value;
                                this.mark_changed(cx);
                            },
                            cx,
                        )),
                    cx,
                ),
            )
            // UI font settings
            .child(
                self.render_section(
                    "UI Font",
                    "Font used for interface elements",
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(self.render_font_select(
                            "Font family",
                            &self.pending.ui.ui_font_family,
                            &["Inter", "SF Pro", "Helvetica Neue", "System"],
                            |this, value, cx| {
                                this.pending.ui.ui_font_family = value.to_string();
                                this.mark_changed(cx);
                            },
                            cx,
                        ))
                        .child(self.render_slider(
                            "Font size",
                            self.pending.ui.ui_font_size,
                            10.0,
                            20.0,
                            |this, value, cx| {
                                this.pending.ui.ui_font_size = value;
                                this.mark_changed(cx);
                            },
                            cx,
                        )),
                    cx,
                ),
            )
            // Vim mode
            .child(self.render_section(
                "Keybindings",
                "Keyboard shortcuts and editing modes",
                self.render_toggle(
                    "Enable Vim mode",
                    self.pending.editor.vim_mode,
                    |this, cx| {
                        this.pending.editor.vim_mode = !this.pending.editor.vim_mode;
                        this.mark_changed(cx);
                    },
                    cx,
                ),
                cx,
            ))
    }

    /// Render a font select dropdown (simplified as buttons for now)
    pub(crate) fn render_font_select(
        &self,
        _label: &str,
        current: &str,
        options: &[&'static str],
        on_change: impl Fn(&mut Self, &str, &mut Context<Self>) + 'static + Clone,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_wrap()
            .gap_1()
            .children(options.iter().map(|&option| {
                let is_selected = current == option;
                let on_change = on_change.clone();

                div()
                    .id(SharedString::from(format!("font-{}", option)))
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_xs()
                    .when(is_selected, |d| {
                        d.bg(theme.colors.accent)
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                    })
                    .when(!is_selected, |d| {
                        d.bg(theme.colors.surface_hover)
                            .text_color(theme.colors.text_muted)
                            .hover(|s| s.text_color(theme.colors.text))
                    })
                    .child(option)
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        on_change(this, option, cx);
                    }))
            }))
    }
}
