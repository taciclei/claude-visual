//! Model switching and connection status functionality for ChatView

use super::core::ChatView;
use super::types::{ChatViewEvent, ConnectionStatus, ModelInfo};
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    // ==================== Model Methods ====================

    /// Set the current model
    pub fn set_model(&mut self, model_id: &str, cx: &mut Context<Self>) {
        for model in &mut self.available_models {
            model.is_current = model.id == model_id;
        }
        cx.notify();
    }

    /// Update the list of available models
    pub fn update_available_models(&mut self, models: Vec<ModelInfo>, cx: &mut Context<Self>) {
        self.available_models = models;
        cx.notify();
    }
    /// Toggle model switcher dropdown
    pub fn toggle_model_switcher(&mut self, cx: &mut Context<Self>) {
        self.panels.model_switcher = !self.panels.model_switcher;
        cx.notify();
    }

    /// Select a model and send the /model command
    pub fn select_model(&mut self, model_id: &str, cx: &mut Context<Self>) {
        self.panels.model_switcher = false;
        // Update available_models to mark the selected one as current
        for model in &mut self.available_models {
            model.is_current = model.id == model_id;
        }
        // Send the model switch command
        cx.emit(ChatViewEvent::Submit(format!("/model {}", model_id)));
        cx.notify();
    }

    /// Get available models
    pub fn get_available_models(&self) -> &[ModelInfo] {
        &self.available_models
    }

    /// Get current model info
    pub fn get_current_model(&self) -> Option<&ModelInfo> {
        self.available_models.iter().find(|m| m.is_current)
    }

    // ==================== Connection Methods ====================

    /// Set connection status
    pub fn set_connection_status(&mut self, status: ConnectionStatus, cx: &mut Context<Self>) {
        self.connection_status = status;
        cx.notify();
    }

    /// Get connection status
    pub fn get_connection_status(&self) -> ConnectionStatus {
        self.connection_status
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        matches!(self.connection_status, ConnectionStatus::Connected)
    }

    /// Update response latency
    pub fn update_latency(&mut self, latency_ms: u64, cx: &mut Context<Self>) {
        self.streaming.last_response_time_ms = Some(latency_ms);
        cx.notify();
    }

    // ==================== Rendering ====================

    /// Render model switcher dropdown
    pub fn render_model_switcher(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        let current_model = self
            .session_info
            .as_ref()
            .map(|i| i.model.as_str())
            .unwrap_or("Unknown");

        div()
            .absolute()
            .top(px(40.0))
            .left(px(120.0))
            .w(px(280.0))
            .rounded_lg()
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.border)
            .shadow_lg()
            .overflow_hidden()
            // Header
            .child(
                div()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.colors.text)
                                    .child("Switch Model"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child(format!("Current: {}", current_model)),
                            ),
                    ),
            )
            // Model list
            .child(
                div()
                    .flex()
                    .flex_col()
                    .children(self.available_models.iter().map(|model| {
                        let model_id = model.id.clone();
                        let is_current = model.is_current || current_model.contains(&model.name);
                        div()
                            .id(ElementId::Name(format!("model-{}", model.id).into()))
                            .px_3()
                            .py_2()
                            .flex()
                            .items_center()
                            .gap_2()
                            .cursor_pointer()
                            .bg(if is_current {
                                theme.colors.accent.opacity(0.1)
                            } else {
                                theme.colors.surface
                            })
                            .hover(|s| s.bg(theme.colors.surface_hover))
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.select_model(&model_id, cx);
                            }))
                            // Icon
                            .child(div().w(px(24.0)).text_center().child(model.icon))
                            // Model info
                            .child(
                                div()
                                    .flex_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(if is_current {
                                                theme.colors.accent
                                            } else {
                                                theme.colors.text
                                            })
                                            .child(model.name.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(model.description),
                                    ),
                            )
                            // Current indicator
                            .when(is_current, |d| {
                                d.child(div().text_xs().text_color(theme.colors.success).child("✓"))
                            })
                    })),
            )
            // Model recommendations
            .child(
                div()
                    .px_3()
                    .py_2()
                    .border_t_1()
                    .border_color(theme.colors.border)
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text_muted)
                            .child("💡 When to use each model:"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted.opacity(0.8))
                            .child("• Opus: Complex tasks, /apex, /ultrathink"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted.opacity(0.8))
                            .child("• Sonnet: Balanced, /explore, /review"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted.opacity(0.8))
                            .child("• Haiku: Quick tasks, /search, /commit"),
                    ),
            )
    }

    /// Get model recommendation based on skill
    pub fn get_recommended_model_for_skill(skill: &str) -> &'static str {
        let sl = skill.to_lowercase();
        if sl.contains("apex")
            || sl.contains("ultrathink")
            || sl.contains("brainstorm")
            || sl.contains("architect")
            || sl.contains("plan")
        {
            "opus"
        } else if sl.contains("search")
            || sl.contains("commit")
            || sl.contains("quick")
            || sl.contains("oneshot")
        {
            "haiku"
        } else {
            "sonnet"
        }
    }
}
