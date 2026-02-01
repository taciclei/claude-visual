//! Render implementation for history sidebar

use gpui::prelude::*;
use gpui::*;

use super::core::HistorySidebar;
use super::types::DisplayMode;

mod content;
mod filters;
mod header;

impl Render for HistorySidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let display_mode = self.display_mode;

        // Pre-compute conversation items for recent view
        let conversation_items: Vec<_> = self
            .conversations
            .iter()
            .map(|conv| {
                let is_selected = self.selected_conversation.as_deref() == Some(&conv.id);
                (
                    conv.id.clone(),
                    conv.title.clone(),
                    Self::format_relative_time(&conv.updated_at),
                    is_selected,
                )
            })
            .collect();

        // Pre-compute search results
        let search_items: Vec<_> = self
            .search_results
            .iter()
            .map(|result| {
                (
                    result.message.conversation_id.clone(),
                    result.conversation_title.clone(),
                    Self::truncate(&result.message.content, 100),
                    result.highlighted.clone(),
                    Self::format_relative_time(&result.message.timestamp),
                )
            })
            .collect();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.colors.surface)
            .child(self.render_header(_window, cx))
            .child(self.render_content(conversation_items, search_items, display_mode, cx))
    }
}
