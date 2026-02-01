//! Quick settings panel render functions

use gpui::prelude::*;
use gpui::*;

use super::super::core::ChatView;

impl ChatView {
    pub fn render_quick_settings_panel(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id("quick-settings-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_quick_settings(cx);
            }))
            .child(
                div()
                    .id("quick-settings-panel")
                    .w(px(350.0))
                    .bg(theme.colors.surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_click(|_, _, _| {})
                    // Header
                    .child(
                        div()
                            .px_4()
                            .py_3()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().text_base().child("⚙️"))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child("Quick Settings"),
                                    ),
                            )
                            .child(
                                div()
                                    .id("close-quick-settings")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_quick_settings(cx);
                                    }))
                                    .child("×"),
                            ),
                    )
                    // Settings list
                    .child(
                        div()
                            .id("quick-settings-content")
                            .p_2()
                            .overflow_y_scroll()
                            .max_h(px(400.0))
                            // Display section
                            .child(
                                div().px_3().py_1().child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.colors.text_muted)
                                        .child("DISPLAY"),
                                ),
                            )
                            // Show Timestamps
                            .child(self.render_setting_toggle(
                                "Show Timestamps",
                                "Display message timestamps",
                                self.show_timestamps,
                                "toggle_timestamps",
                                &theme,
                                cx,
                            ))
                            // Time Separators
                            .child(self.render_setting_toggle(
                                "Time Separators",
                                "Show date separators between messages",
                                self.show_time_separators,
                                "toggle_time_separators",
                                &theme,
                                cx,
                            ))
                            // Compact Mode
                            .child(self.render_setting_toggle(
                                "Compact Mode",
                                "Reduce message spacing",
                                self.compact_mode,
                                "toggle_compact",
                                &theme,
                                cx,
                            ))
                            // Line Numbers
                            .child(self.render_setting_toggle(
                                "Line Numbers",
                                "Show line numbers in code",
                                self.show_line_numbers,
                                "toggle_line_numbers",
                                &theme,
                                cx,
                            ))
                            // Word Wrap
                            .child(self.render_setting_toggle(
                                "Word Wrap",
                                "Wrap long lines in code",
                                self.word_wrap,
                                "toggle_word_wrap",
                                &theme,
                                cx,
                            ))
                            // Behavior section
                            .child(
                                div().px_3().py_1().mt_2().child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.colors.text_muted)
                                        .child("BEHAVIOR"),
                                ),
                            )
                            // Auto-scroll
                            .child(self.render_setting_toggle(
                                "Auto-scroll",
                                "Scroll to new messages",
                                self.auto_scroll,
                                "toggle_auto_scroll",
                                &theme,
                                cx,
                            ))
                            // Show Suggestions
                            .child(self.render_setting_toggle(
                                "Suggestions",
                                "Show contextual suggestions",
                                self.panels.suggestions,
                                "toggle_suggestions",
                                &theme,
                                cx,
                            ))
                            // Focus Mode
                            .child(self.render_setting_toggle(
                                "Focus Mode",
                                "Distraction-free input",
                                self.focus_mode,
                                "toggle_focus",
                                &theme,
                                cx,
                            ))
                            // Input Hints
                            .child(self.render_setting_toggle(
                                "Input Hints",
                                "Show input suggestions",
                                self.show_input_hints,
                                "toggle_hints",
                                &theme,
                                cx,
                            ))
                            // Claude section
                            .child(
                                div().px_3().py_1().mt_2().child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.colors.text_muted)
                                        .child("CLAUDE"),
                                ),
                            )
                            // Show Thinking
                            .child(self.render_setting_toggle(
                                "Show Thinking",
                                "Display Claude's reasoning process",
                                self.show_thinking,
                                "toggle_thinking",
                                &theme,
                                cx,
                            ))
                            // Skills & Commands section
                            .child(
                                div().px_3().py_1().mt_2().child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.colors.text_muted)
                                        .child("QUICK ACCESS"),
                                ),
                            )
                            // Quick skill buttons
                            .child(self.render_quick_skill_buttons(&theme, cx)),
                    ),
            )
    }

    /// Render quick skill access buttons
    fn render_quick_skill_buttons(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        use super::super::types::ChatViewEvent;

        div()
            .px_3()
            .py_2()
            .flex()
            .flex_col()
            .gap_2()
            // Top skills row
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_2()
                    // APEX
                    .child(
                        div()
                            .id("qs-apex")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(theme.colors.accent.opacity(0.1))
                            .border_1()
                            .border_color(theme.colors.accent.opacity(0.2))
                            .text_xs()
                            .text_color(theme.colors.accent)
                            .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_quick_settings(cx);
                                cx.emit(ChatViewEvent::Submit("/apex".to_string()));
                            }))
                            .child("⚡")
                            .child("APEX"),
                    )
                    // Explore
                    .child(
                        div()
                            .id("qs-explore")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(theme.colors.info.opacity(0.1))
                            .border_1()
                            .border_color(theme.colors.info.opacity(0.2))
                            .text_xs()
                            .text_color(theme.colors.info)
                            .hover(|s| s.bg(theme.colors.info.opacity(0.2)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_quick_settings(cx);
                                cx.emit(ChatViewEvent::Submit("/explore".to_string()));
                            }))
                            .child("🔍")
                            .child("Explore"),
                    )
                    // Debug
                    .child(
                        div()
                            .id("qs-debug")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(theme.colors.error.opacity(0.1))
                            .border_1()
                            .border_color(theme.colors.error.opacity(0.2))
                            .text_xs()
                            .text_color(theme.colors.error)
                            .hover(|s| s.bg(theme.colors.error.opacity(0.2)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_quick_settings(cx);
                                cx.emit(ChatViewEvent::Submit("/debug".to_string()));
                            }))
                            .child("🐛")
                            .child("Debug"),
                    )
                    // Commit
                    .child(
                        div()
                            .id("qs-commit")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(theme.colors.success.opacity(0.1))
                            .border_1()
                            .border_color(theme.colors.success.opacity(0.2))
                            .text_xs()
                            .text_color(theme.colors.success)
                            .hover(|s| s.bg(theme.colors.success.opacity(0.2)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_quick_settings(cx);
                                cx.emit(ChatViewEvent::Submit("/commit".to_string()));
                            }))
                            .child("📦")
                            .child("Commit"),
                    ),
            )
            // Bottom skills row
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_2()
                    // Review
                    .child(
                        div()
                            .id("qs-review")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(theme.colors.warning.opacity(0.1))
                            .border_1()
                            .border_color(theme.colors.warning.opacity(0.2))
                            .text_xs()
                            .text_color(theme.colors.warning)
                            .hover(|s| s.bg(theme.colors.warning.opacity(0.2)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_quick_settings(cx);
                                cx.emit(ChatViewEvent::Submit("/review".to_string()));
                            }))
                            .child("👀")
                            .child("Review"),
                    )
                    // Refactor
                    .child(
                        div()
                            .id("qs-refactor")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(theme.colors.surface_hover)
                            .border_1()
                            .border_color(theme.colors.border)
                            .text_xs()
                            .text_color(theme.colors.text)
                            .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_quick_settings(cx);
                                cx.emit(ChatViewEvent::Submit("/refactor".to_string()));
                            }))
                            .child("🔄")
                            .child("Refactor"),
                    )
                    // Oneshot
                    .child(
                        div()
                            .id("qs-oneshot")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(theme.colors.surface_hover)
                            .border_1()
                            .border_color(theme.colors.border)
                            .text_xs()
                            .text_color(theme.colors.text)
                            .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_quick_settings(cx);
                                cx.emit(ChatViewEvent::Submit("/oneshot".to_string()));
                            }))
                            .child("🚀")
                            .child("Oneshot"),
                    )
                    // Ultrathink
                    .child(
                        div()
                            .id("qs-ultrathink")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(theme.colors.surface_hover)
                            .border_1()
                            .border_color(theme.colors.border)
                            .text_xs()
                            .text_color(theme.colors.text)
                            .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_quick_settings(cx);
                                cx.emit(ChatViewEvent::Submit("/ultrathink".to_string()));
                            }))
                            .child("🧠")
                            .child("Ultrathink"),
                    ),
            )
    }

    /// Render a setting toggle row
    pub fn render_setting_toggle(
        &self,
        label: &str,
        description: &str,
        is_on: bool,
        action: &'static str,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id(ElementId::Name(format!("setting-{}", action).into()))
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_2()
            .rounded_md()
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.surface_hover))
            .on_click(cx.listener(move |this, _, _window, cx| match action {
                "toggle_timestamps" => this.toggle_timestamps(cx),
                "toggle_time_separators" => this.toggle_time_separators(cx),
                "toggle_compact" => this.toggle_compact_mode(cx),
                "toggle_focus" => this.toggle_focus_mode(cx),
                "toggle_hints" => this.toggle_input_hints(cx),
                "toggle_line_numbers" => this.toggle_line_numbers(cx),
                "toggle_word_wrap" => this.toggle_word_wrap(cx),
                "toggle_auto_scroll" => this.toggle_auto_scroll(cx),
                "toggle_suggestions" => this.toggle_suggestions(cx),
                "toggle_thinking" => this.toggle_thinking(cx),
                _ => {}
            }))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text)
                            .child(label.to_string()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(description.to_string()),
                    ),
            )
            .child(
                div()
                    .w(px(40.0))
                    .h(px(22.0))
                    .rounded_full()
                    .bg(if is_on {
                        theme.colors.accent
                    } else {
                        theme.colors.surface_hover
                    })
                    .border_1()
                    .border_color(if is_on {
                        theme.colors.accent
                    } else {
                        theme.colors.border
                    })
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .w(px(18.0))
                            .h(px(18.0))
                            .rounded_full()
                            .bg(theme.colors.surface)
                            .ml(if is_on { px(20.0) } else { px(1.0) }),
                    ),
            )
    }
}
