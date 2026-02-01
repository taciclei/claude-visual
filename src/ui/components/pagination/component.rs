//! Pagination component struct and methods

use std::sync::Arc;

use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// Pagination component
pub struct Pagination {
    pub(super) app_state: Arc<AppState>,
    /// Current page (1-indexed)
    pub(super) current_page: usize,
    /// Total number of pages
    pub(super) total_pages: usize,
    /// Number of visible page buttons
    pub(super) visible_pages: usize,
    /// Size variant
    pub(super) size: PaginationSize,
    /// Style variant
    pub(super) style: PaginationStyle,
    /// Show first/last buttons
    pub(super) show_first_last: bool,
    /// Show prev/next buttons
    pub(super) show_prev_next: bool,
    /// Disabled state
    pub(super) disabled: bool,
}

impl Pagination {
    pub fn new(app_state: Arc<AppState>, total_pages: usize, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            current_page: 1,
            total_pages,
            visible_pages: 5,
            size: PaginationSize::default(),
            style: PaginationStyle::default(),
            show_first_last: false,
            show_prev_next: true,
            disabled: false,
        }
    }

    /// Set current page
    pub fn set_page(&mut self, page: usize, cx: &mut Context<Self>) {
        let new_page = page.clamp(1, self.total_pages.max(1));
        if new_page != self.current_page {
            self.current_page = new_page;
            cx.emit(PaginationEvent::PageChanged(new_page));
            cx.notify();
        }
    }

    /// Set total pages
    pub fn set_total_pages(&mut self, total: usize, cx: &mut Context<Self>) {
        self.total_pages = total;
        if self.current_page > total {
            self.current_page = total.max(1);
        }
        cx.notify();
    }

    /// Set visible pages
    pub fn set_visible_pages(&mut self, visible: usize, cx: &mut Context<Self>) {
        self.visible_pages = visible.max(1);
        cx.notify();
    }

    /// Set size
    pub fn set_size(&mut self, size: PaginationSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set style
    pub fn set_style(&mut self, style: PaginationStyle, cx: &mut Context<Self>) {
        self.style = style;
        cx.notify();
    }

    /// Set show first/last
    pub fn set_show_first_last(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_first_last = show;
        cx.notify();
    }

    /// Set show prev/next
    pub fn set_show_prev_next(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_prev_next = show;
        cx.notify();
    }

    /// Set disabled
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }

    /// Go to first page
    pub fn first(&mut self, cx: &mut Context<Self>) {
        self.set_page(1, cx);
    }

    /// Go to last page
    pub fn last(&mut self, cx: &mut Context<Self>) {
        self.set_page(self.total_pages, cx);
    }

    /// Go to previous page
    pub fn prev(&mut self, cx: &mut Context<Self>) {
        if self.current_page > 1 {
            self.set_page(self.current_page - 1, cx);
        }
    }

    /// Go to next page
    pub fn next(&mut self, cx: &mut Context<Self>) {
        if self.current_page < self.total_pages {
            self.set_page(self.current_page + 1, cx);
        }
    }

    /// Get current page
    pub fn current_page(&self) -> usize {
        self.current_page
    }

    /// Calculate visible page range
    pub(super) fn visible_range(&self) -> Vec<Option<usize>> {
        let total = self.total_pages;
        let current = self.current_page;
        let visible = self.visible_pages;

        if total <= visible {
            return (1..=total).map(Some).collect();
        }

        let half = visible / 2;
        let start = if current <= half + 1 {
            1
        } else if current >= total - half {
            total - visible + 1
        } else {
            current - half
        };

        let end = (start + visible - 1).min(total);

        let mut pages: Vec<Option<usize>> = Vec::new();

        // Add first page and ellipsis if needed
        if start > 1 {
            pages.push(Some(1));
            if start > 2 {
                pages.push(None); // Ellipsis
            }
        }

        // Add visible pages
        for page in start..=end {
            pages.push(Some(page));
        }

        // Add ellipsis and last page if needed
        if end < total {
            if end < total - 1 {
                pages.push(None); // Ellipsis
            }
            pages.push(Some(total));
        }

        pages
    }
}

impl EventEmitter<PaginationEvent> for Pagination {}
