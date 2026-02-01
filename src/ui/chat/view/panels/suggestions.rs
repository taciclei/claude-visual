//! Suggestions panel render functions

use gpui::*;
use gpui::prelude::*;

use crate::ui::pct;
use super::super::core::ChatView;

impl ChatView {
    /// Render contextual suggestions bar
    pub fn render_suggestions_bar(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> Div {
        let suggestions: Vec<_> = self.contextual_suggestions.iter().enumerate().collect();

        div()
            .w_full()
            .px_4()
            .py_2()
            .border_t_1()
            .border_color(theme.colors.border)
            .bg(theme.colors.surface.opacity(0.5))
            .flex()
            .items_center()
            .gap_2()
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child("💡")
                    .child("Suggestions:")
            )
            // Suggestion chips
            .children(suggestions.into_iter().map(|(idx, suggestion)| {
                let text = suggestion.text.clone();
                div()
                    .id(SharedString::from(format!("suggestion-{}", idx)))
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .bg(theme.colors.accent.opacity(0.1))
                    .border_1()
                    .border_color(theme.colors.accent.opacity(0.2))
                    .cursor_pointer()
                    .text_xs()
                    .text_color(theme.colors.text)
                    .hover(|s| {
                        s.bg(theme.colors.accent.opacity(0.2))
                            .border_color(theme.colors.accent.opacity(0.4))
                    })
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.use_suggestion(idx, cx);
                    }))
                    .child(suggestion.icon)
                    .child(text)
            }))
            // Toggle button
            .child(
                div()
                    .ml_auto()
                    .id("toggle-suggestions")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .hover(|s| s.bg(theme.colors.surface_hover))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle_suggestions(cx);
                    }))
                    .child("Hide")
            )
    }

    /// Render quick access templates bar (shows top 5 most used templates)
    pub fn render_quick_templates_bar(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        // Get top 5 most used templates (sorted by usage count)
        let mut templates: Vec<_> = self.prompt_templates.iter()
            .filter(|t| t.usage_count > 0 || t.is_builtin)
            .collect();
        templates.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
        let top_templates: Vec<_> = templates.into_iter().take(5).collect();

        div()
            .id("quick-templates-bar")
            .w_full()
            .px_4()
            .py_1()
            .border_t_1()
            .border_color(theme.colors.border.opacity(0.5))
            .bg(theme.colors.surface.opacity(0.3))
            .flex()
            .items_center()
            .gap_2()
            .overflow_x_scroll()
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .flex_shrink_0()
                    .child("📝")
                    .child("Quick:")
            )
            // Template chips
            .children(top_templates.into_iter().map(|template| {
                let template_id = template.id.clone();
                div()
                    .id(ElementId::Name(format!("quick-template-{}", template.id).into()))
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py(px(3.0))
                    .rounded_md()
                    .bg(theme.colors.background)
                    .border_1()
                    .border_color(theme.colors.border.opacity(0.5))
                    .cursor_pointer()
                    .text_xs()
                    .text_color(theme.colors.text)
                    .flex_shrink_0()
                    .hover(|s| {
                        s.bg(theme.colors.accent.opacity(0.1))
                            .border_color(theme.colors.accent.opacity(0.3))
                    })
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.use_template(&template_id, cx);
                    }))
                    .child(template.icon)
                    .child(template.name.clone())
            }))
            // Spacer
            .child(div().flex_1())
            // Open all templates button
            .child(
                div()
                    .id("open-all-templates")
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py(px(3.0))
                    .rounded_md()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(theme.colors.accent)
                    .flex_shrink_0()
                    .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle_templates_panel(cx);
                    }))
                    .child("All templates")
                    .child("→")
            )
    }
}
