//! Commands panel render functions
//!
//! This module is organized into submodules:
//! - **grouping** - Skill categorization logic
//! - **header** - Header rendering with search and category tabs
//! - **content** - Content rendering (slash commands and skills)
//! - **footer** - Footer rendering with shortcuts

use gpui::*;
use gpui::prelude::*;

use super::super::core::ChatView;
use super::super::types::CommandCategory;

mod grouping;
mod header;
mod content;
mod footer;

impl ChatView {
    pub fn render_commands_panel(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let (slash_commands, skills) = self.filtered_commands();
        let filter = self.commands_filter.clone();
        let category = self.commands_category;

        // Group skills by category
        let skill_categories = grouping::group_skills_by_category(&skills);

        // Extract listeners before div chain
        let on_overlay_click = cx.listener(|this, _, _window, cx| {
            this.toggle_commands_panel(cx);
        });

        div()
            .id("commands-panel-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(on_overlay_click)
            .child(
                div()
                    .id("commands-panel")
                    .w(px(550.0))
                    .max_h(px(500.0))
                    .bg(theme.colors.surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_click(|_, _, _| {})
                    .child(header::render_header(theme, &filter, category, slash_commands.len() + skills.len(), cx))
                    .child(content::render_content(theme, category, &slash_commands, &skills, &skill_categories, cx))
                    .child(footer::render_footer(theme))
            )
    }
}
