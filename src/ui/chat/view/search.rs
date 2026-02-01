//! Search functionality for ChatView

use gpui::*;

use super::core::ChatView;
use super::types::{ConversationSearchResult, MessageFilter, NotificationType};
use crate::claude::message::MessageRole;

impl ChatView {
    /// Toggle search panel visibility
    pub fn toggle_search(&mut self, cx: &mut Context<Self>) {
        self.search.show = !self.search.show;
        if !self.search.show {
            // Clear search when hiding
            self.search.query.clear();
            self.search.results.clear();
            self.search.current_result = 0;
        }
        cx.notify();
    }

    /// Set search query and perform search
    pub fn set_search_query(&mut self, query: String, cx: &mut Context<Self>) {
        self.search.query = query;
        self.perform_search();
        self.search.current_result = 0;
        cx.notify();
    }

    // Core search logic moved to search_logic.rs

    /// Navigate to next search result and scroll to it
    pub fn next_search_result(&mut self, cx: &mut Context<Self>) {
        if !self.search.results.is_empty() {
            self.search.current_result =
                (self.search.current_result + 1) % self.search.results.len();
            self.scroll_to_search_result(cx);
        }
    }

    /// Navigate to previous search result and scroll to it
    pub fn prev_search_result(&mut self, cx: &mut Context<Self>) {
        if !self.search.results.is_empty() {
            if self.search.current_result == 0 {
                self.search.current_result = self.search.results.len() - 1;
            } else {
                self.search.current_result -= 1;
            }
            self.scroll_to_search_result(cx);
        }
    }

    /// Get current search result
    pub fn current_result(&self) -> Option<&ConversationSearchResult> {
        self.search.results.get(self.search.current_result)
    }

    /// Get search results count
    pub fn search_result_count(&self) -> usize {
        self.search.results.len()
    }

    /// Get current search result index (1-based for display)
    pub fn current_result_index(&self) -> usize {
        if self.search.results.is_empty() {
            0
        } else {
            self.search.current_result + 1
        }
    }

    /// Jump to a specific search result by index
    pub fn jump_to_search_result(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.search.results.len() {
            self.search.current_result = index;
            self.scroll_to_search_result(cx);
        }
    }

    /// Toggle case-sensitive search
    pub fn toggle_search_case_sensitive(&mut self, cx: &mut Context<Self>) {
        self.search.case_sensitive = !self.search.case_sensitive;
        self.perform_search();
        self.search.current_result = 0;
        cx.notify();
    }

    /// Toggle regex search
    pub fn toggle_search_regex(&mut self, cx: &mut Context<Self>) {
        self.search.regex = !self.search.regex;
        self.perform_search();
        self.search.current_result = 0;
        cx.notify();
    }

    /// Set search role filter
    pub fn set_search_role_filter(&mut self, filter: MessageFilter, cx: &mut Context<Self>) {
        self.search.role_filter = filter;
        self.perform_search();
        self.search.current_result = 0;
        cx.notify();
    }

    /// Cycle through search role filters
    pub fn cycle_search_role_filter(&mut self, cx: &mut Context<Self>) {
        self.search.role_filter = match self.search.role_filter {
            MessageFilter::All => MessageFilter::UserOnly,
            MessageFilter::UserOnly => MessageFilter::AssistantOnly,
            MessageFilter::AssistantOnly => MessageFilter::ToolsOnly,
            MessageFilter::ToolsOnly => MessageFilter::All,
        };
        self.perform_search();
        self.search.current_result = 0;
        cx.notify();
    }
}
