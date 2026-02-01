//! Toolbar with quick action buttons

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;

use super::ChatInput;
use super::types::ChatInputEvent;

impl ChatInput {
    /// Render toolbar with attach, command, templates, skills, and think mode buttons
    pub(super) fn render_toolbar(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_1()
            .items_center()
            // Attach file button
            .child(self.render_attach_button(theme, cx))
            // Slash command button
            .child(self.render_command_button(theme, cx))
            // Quick skills buttons
            .child(self.render_quick_skills(theme, cx))
            // Templates button
            .child(self.render_templates_button(theme, cx))
            // Think mode toggle
            .child(self.render_think_button(theme, cx))
    }

    /// Render quick skill buttons (most used skills)
    fn render_quick_skills(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        // Primary skills with color coding
        let primary_skills: [(&str, &str, gpui::Hsla); 4] = [
            ("⚡", "apex", theme.colors.accent),
            ("🔍", "explore", theme.colors.info),
            ("🐛", "debug", theme.colors.error),
            ("📦", "commit", theme.colors.success),
        ];

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_px()
            // Primary skill buttons
            .children(primary_skills.iter().map(|(icon, skill, color)| {
                let skill_cmd = format!("/{}", skill);
                let btn_color = *color;
                div()
                    .id(SharedString::from(format!("quick-skill-{}", skill)))
                    .size(px(28.0))
                    .rounded_md()
                    .bg(btn_color.opacity(0.08))
                    .border_1()
                    .border_color(btn_color.opacity(0.2))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|s| {
                        s.bg(btn_color.opacity(0.2))
                            .border_color(btn_color.opacity(0.5))
                    })
                    .on_click(cx.listener(move |_this, _, _window, cx| {
                        cx.emit(ChatInputEvent::Submit(skill_cmd.clone()));
                    }))
                    .child(
                        div()
                            .text_xs()
                            .text_color(btn_color)
                            .child(*icon)
                    )
            }))
            // Separator
            .child(
                div()
                    .w(px(20.0))
                    .h_px()
                    .bg(theme.colors.border.opacity(0.5))
                    .my_1()
            )
            // Secondary skills (more subtle styling)
            .child(self.render_secondary_skills(theme, cx))
    }

    /// Render secondary skill buttons
    fn render_secondary_skills(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let secondary_skills = [
            ("👀", "review"),
            ("🚀", "oneshot"),
        ];

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_px()
            .children(secondary_skills.iter().map(|(icon, skill)| {
                let skill_cmd = format!("/{}", skill);
                div()
                    .id(SharedString::from(format!("quick-skill-{}", skill)))
                    .size(px(26.0))
                    .rounded_md()
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(theme.colors.border.opacity(0.5))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|s| {
                        s.bg(theme.colors.surface_hover)
                            .border_color(theme.colors.accent.opacity(0.3))
                    })
                    .on_click(cx.listener(move |_this, _, _window, cx| {
                        cx.emit(ChatInputEvent::Submit(skill_cmd.clone()));
                    }))
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(*icon)
                    )
            }))
    }

    /// Render attach file button with keyboard hint
    fn render_attach_button(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_1()
            .child(
                div()
                    .id("attach-button")
                    .size(px(32.0))
                    .rounded_md()
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(theme.colors.border)
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|s| {
                        s.bg(theme.colors.surface_hover)
                            .border_color(theme.colors.accent.opacity(0.5))
                    })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        // Insert @file: mention template
                        this.text.push_str("@file:");
                        this.cursor_position = this.text.len();
                        this.update_mentions();
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child("📎")
                    )
            )
            // Keyboard hint: @ for mentions
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .font_family("monospace")
                    .child("@")
            )
    }

    /// Render slash command button with keyboard hint
    fn render_command_button(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_1()
            .child(
                div()
                    .id("command-button")
                    .size(px(32.0))
                    .rounded_md()
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(theme.colors.border)
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|s| {
                        s.bg(theme.colors.surface_hover)
                            .border_color(theme.colors.accent.opacity(0.5))
                    })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        // Insert slash to trigger command autocomplete
                        if !this.text.starts_with('/') {
                            this.text = "/".to_string() + &this.text;
                            this.cursor_position = 1;
                        }
                        this.update_command_autocomplete();
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .font_family("monospace")
                            .child("/")
                    )
            )
            // Keyboard hint: / for commands
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .font_family("monospace")
                    .child("/")
            )
    }

    /// Render templates button
    fn render_templates_button(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let is_open = self.show_templates;

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_1()
            .child(
                div()
                    .id("templates-button")
                    .size(px(32.0))
                    .rounded_md()
                    .bg(if is_open { theme.colors.accent.opacity(0.2) } else { theme.colors.surface })
                    .border_1()
                    .border_color(if is_open { theme.colors.accent.opacity(0.5) } else { theme.colors.border })
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|s| {
                        s.bg(theme.colors.surface_hover)
                            .border_color(theme.colors.accent.opacity(0.5))
                    })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle_templates(cx);
                    }))
                    .child(
                        div()
                            .text_sm()
                            .text_color(if is_open { theme.colors.accent } else { theme.colors.text_muted })
                            .child("📋")
                    )
            )
            // Keyboard hint
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .font_family("monospace")
                    .child("T")
            )
    }

    /// Render think mode toggle button
    fn render_think_button(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let is_enabled = self.think_mode_enabled;
        let button_bg = if is_enabled {
            theme.colors.warning.opacity(0.2)
        } else {
            theme.colors.surface
        };
        let border_color = if is_enabled {
            theme.colors.warning.opacity(0.5)
        } else {
            theme.colors.border
        };
        let icon_color = if is_enabled {
            theme.colors.warning
        } else {
            theme.colors.text_muted
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_1()
            .child(
                div()
                    .id("think-button")
                    .size(px(32.0))
                    .rounded_md()
                    .bg(button_bg)
                    .border_1()
                    .border_color(border_color)
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|s| {
                        s.bg(theme.colors.warning.opacity(0.15))
                            .border_color(theme.colors.warning.opacity(0.5))
                    })
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(ChatInputEvent::ToggleThinkMode);
                    }))
                    .child(
                        div()
                            .text_sm()
                            .text_color(icon_color)
                            .child("🧠")
                    )
            )
            // Status hint
            .child(
                div()
                    .text_xs()
                    .text_color(if is_enabled { theme.colors.warning } else { theme.colors.text_muted })
                    .font_family("monospace")
                    .child(if is_enabled { "ON" } else { "—" })
            )
    }
}
