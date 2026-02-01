//! Template management methods for ChatView

use gpui::*;
use super::core::ChatView;
use super::types::{PromptTemplate, NotificationType};

impl ChatView {
    // ==================== Prompt Templates ====================

    /// Toggle templates panel
    pub fn toggle_templates_panel(&mut self, cx: &mut Context<Self>) {
        self.panels.templates_panel = !self.panels.templates_panel;
        if !self.panels.templates_panel {
            self.templates_filter.clear();
        }
        cx.notify();
    }

    /// Set templates filter
    pub fn set_templates_filter(&mut self, filter: String, cx: &mut Context<Self>) {
        self.templates_filter = filter;
        cx.notify();
    }

    /// Get filtered templates
    pub fn filtered_templates(&self) -> Vec<&PromptTemplate> {
        let filter = self.templates_filter.to_lowercase();
        self.prompt_templates
            .iter()
            .filter(|t| {
                filter.is_empty() ||
                t.name.to_lowercase().contains(&filter) ||
                t.content.to_lowercase().contains(&filter) ||
                t.category.to_lowercase().contains(&filter)
            })
            .collect()
    }

    /// Use a template - insert its content into input
    pub fn use_template(&mut self, template_id: &str, cx: &mut Context<Self>) {
        if let Some(template) = self.prompt_templates.iter_mut().find(|t| t.id == template_id) {
            template.usage_count += 1;
            let content = template.content.clone();
            self.input.update(cx, |input, cx| {
                input.insert_text(&content, cx);
            });
        }
        self.panels.templates_panel = false;
        cx.notify();
    }

    /// Save current input as a new template
    pub fn save_as_template(&mut self, name: String, cx: &mut Context<Self>) {
        let content = self.input.read(cx).text();
        if !content.is_empty() && !name.is_empty() {
            let template = PromptTemplate::new(name, content, "custom", "📌");
            self.prompt_templates.push(template);
            self.show_notification("Template saved!", NotificationType::Success, cx);
        }
        cx.notify();
    }

    /// Delete a custom template
    pub fn delete_template(&mut self, template_id: &str, cx: &mut Context<Self>) {
        self.prompt_templates.retain(|t| t.id != template_id || t.is_builtin);
        cx.notify();
    }

    /// Get templates grouped by category
    pub fn templates_by_category(&self) -> std::collections::HashMap<&'static str, Vec<&PromptTemplate>> {
        let mut groups: std::collections::HashMap<&'static str, Vec<&PromptTemplate>> = std::collections::HashMap::new();
        let filter = self.templates_filter.to_lowercase();

        for template in &self.prompt_templates {
            if filter.is_empty() ||
               template.name.to_lowercase().contains(&filter) ||
               template.content.to_lowercase().contains(&filter) {
                groups.entry(template.category).or_default().push(template);
            }
        }

        groups
    }
}
