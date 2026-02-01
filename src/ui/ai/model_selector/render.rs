use super::colors::default_colors;
use super::core::ModelSelector;
use super::types::{ModelCategory, SimpleColors};
use crate::ai::provider::ModelInfo;
use gpui::prelude::*;
use gpui::*;

impl Render for ModelSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = default_colors();
        let selected_model = self.selected_model();
        let provider_name = self.selected_provider_name();
        let is_expanded = self.is_expanded;

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .gap_1()
            .child(
                // Selector button
                div()
                    .id("model-selector-button")
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_2()
                    .bg(theme.surface)
                    .border_1()
                    .border_color(theme.border)
                    .rounded_md()
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.surface_hover))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle_expanded(cx);
                    }))
                    .child(
                        // Provider icon placeholder
                        div()
                            .w_4()
                            .h_4()
                            .rounded_sm()
                            .bg(if provider_name == "Claude" {
                                rgb(0xD97706) // Amber for Claude
                            } else if provider_name == "OpenAI" {
                                rgb(0x10A37F) // Green for OpenAI
                            } else {
                                rgb(0x6B7280) // Gray for others
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(
                                div().text_sm().text_color(theme.text).child(
                                    selected_model
                                        .map(|m| m.name.clone())
                                        .unwrap_or_else(|| "Select model".to_string()),
                                ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(provider_name.to_string()),
                            ),
                    )
                    .child(
                        // Chevron
                        div().text_color(theme.text_muted).child(if is_expanded {
                            "▲"
                        } else {
                            "▼"
                        }),
                    ),
            )
            .when(is_expanded, |el| el.child(self.render_dropdown(&theme, cx)))
    }
}

impl ModelSelector {
    pub(crate) fn render_dropdown(
        &self,
        theme: &SimpleColors,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let filtered = self.filtered_models();

        div()
            .absolute()
            .top_full()
            .left_0()
            .right_0()
            .mt_1()
            .bg(theme.surface)
            .border_1()
            .border_color(theme.border)
            .rounded_md()
            .shadow_lg()
            .max_h_80()
            .id("scroll-model-dropdown")
            .overflow_y_scroll()
            .child(
                // Search input
                div()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_muted)
                            .child("Search models..."),
                    ),
            )
            .children(
                // Group by category
                self.render_category_section(
                    "Cloud Models".to_string(),
                    ModelCategory::Cloud,
                    &filtered,
                    &theme,
                    cx,
                ),
            )
            .children(self.render_category_section(
                "Local Models".to_string(),
                ModelCategory::Local,
                &filtered,
                &theme,
                cx,
            ))
    }

    pub(crate) fn render_category_section(
        &self,
        title: String,
        category: ModelCategory,
        filtered: &[(usize, &ModelInfo)],
        theme: &SimpleColors,
        cx: &mut Context<Self>,
    ) -> Option<Div> {
        let category_models: Vec<_> = filtered
            .iter()
            .filter(|(idx, _)| self.providers[*idx].category == category)
            .collect();

        if category_models.is_empty() {
            return None;
        }

        Some(
            div()
                .flex()
                .flex_col()
                .child(
                    // Category header
                    div()
                        .px_3()
                        .py_1()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text_muted)
                        .bg(theme.background)
                        .child(title),
                )
                .children(category_models.into_iter().map(|(provider_idx, model)| {
                    let provider_idx = *provider_idx;
                    let model_id = model.id.clone();
                    let is_selected = self.selected_model_id == model.id
                        && self.selected_provider_idx == provider_idx;
                    let provider = &self.providers[provider_idx];

                    div()
                        .id(SharedString::from(format!(
                            "model-option-{}-{}",
                            provider_idx, model.id
                        )))
                        .px_3()
                        .py_2()
                        .flex()
                        .items_center()
                        .gap_2()
                        .cursor_pointer()
                        .bg(if is_selected {
                            theme.accent.opacity(0.1)
                        } else {
                            theme.surface
                        })
                        .hover(|s| s.bg(theme.surface_hover))
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            this.select_model(provider_idx, model_id.clone(), cx);
                        }))
                        .child(
                            // Model icon/indicator
                            div()
                                .w_2()
                                .h_2()
                                .rounded_full()
                                .bg(if provider.is_configured {
                                    theme.success
                                } else {
                                    theme.text_muted
                                }),
                        )
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(theme.text)
                                        .child(model.name.clone()),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child(format!("{}K context", model.context_length / 1000)),
                                ),
                        )
                        .child(
                            // Cost indicator (for cloud models)
                            div().text_xs().text_color(theme.text_muted).child(
                                model
                                    .input_cost_per_1k
                                    .map(|c| format!("${:.4}/1K", c))
                                    .unwrap_or_else(|| "Free".to_string()),
                            ),
                        )
                        .when(is_selected, |el| {
                            el.child(div().text_color(theme.accent).child("✓"))
                        })
                })),
        )
    }
}
