//! Filter functionality for history sidebar

use gpui::*;

use crate::storage::models::DateRangeFilter;

use super::core::HistorySidebar;
use super::types::DisplayMode;

impl HistorySidebar {
    /// Toggle filter panel visibility
    pub(super) fn toggle_filters(&mut self, cx: &mut Context<Self>) {
        self.show_filters = !self.show_filters;
        cx.notify();
    }

    /// Set date range filter
    pub(super) fn set_date_filter(&mut self, filter: DateRangeFilter, cx: &mut Context<Self>) {
        self.search_filter.date_range = filter;
        if self.display_mode == DisplayMode::Search {
            self.perform_search(cx);
        }
        cx.notify();
    }

    /// Set project filter
    pub(super) fn set_project_filter(
        &mut self,
        project_id: Option<String>,
        cx: &mut Context<Self>,
    ) {
        self.search_filter.project_id = project_id;
        if self.display_mode == DisplayMode::Search {
            self.perform_search(cx);
        }
        cx.notify();
    }

    /// Clear all filters
    pub(super) fn clear_filters(&mut self, cx: &mut Context<Self>) {
        self.search_filter.clear();
        if self.display_mode == DisplayMode::Search {
            self.perform_search(cx);
        }
        cx.notify();
    }
}
