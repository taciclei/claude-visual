//! Pagination state tracking

use super::PageInfo;

/// Pagination state tracker
///
/// Tracks pagination state for a conversation.
#[derive(Debug, Clone)]
pub struct PaginationState {
    /// Current page size
    pub page_size: usize,
    /// Whether currently loading
    pub is_loading: bool,
    /// Whether there are more older messages
    pub has_older: bool,
    /// Whether there are more newer messages
    pub has_newer: bool,
    /// Total message count (if known)
    pub total_count: Option<usize>,
    /// Number of messages loaded
    pub loaded_count: usize,
}

impl PaginationState {
    /// Create a new pagination state
    pub fn new(page_size: usize) -> Self {
        Self {
            page_size,
            is_loading: false,
            has_older: false,
            has_newer: false,
            total_count: None,
            loaded_count: 0,
        }
    }

    /// Check if should load more when scrolling up
    pub fn should_load_older(&self) -> bool {
        !self.is_loading && self.has_older
    }

    /// Check if should load more when scrolling down
    pub fn should_load_newer(&self) -> bool {
        !self.is_loading && self.has_newer
    }

    /// Start loading
    pub fn start_loading(&mut self) {
        self.is_loading = true;
    }

    /// Finish loading
    pub fn finish_loading(&mut self, page_info: &PageInfo, loaded: usize) {
        self.is_loading = false;
        self.has_older = page_info.has_previous_page;
        self.has_newer = page_info.has_next_page;
        self.loaded_count += loaded;
        if let Some(total) = page_info.total_count {
            self.total_count = Some(total);
        }
    }

    /// Get loading progress as percentage (if total known)
    pub fn progress(&self) -> Option<f32> {
        self.total_count
            .map(|total| (self.loaded_count as f32 / total as f32).min(1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_state() {
        let mut state = PaginationState::new(50);

        assert!(!state.is_loading);
        assert!(!state.should_load_older());

        state.has_older = true;
        assert!(state.should_load_older());

        state.start_loading();
        assert!(!state.should_load_older());

        let page_info = PageInfo {
            has_previous_page: false,
            has_next_page: false,
            start_cursor: None,
            end_cursor: None,
            total_count: Some(100),
        };
        state.finish_loading(&page_info, 50);

        assert!(!state.is_loading);
        assert!(!state.has_older);
        assert_eq!(state.loaded_count, 50);
        assert_eq!(state.progress(), Some(0.5));
    }
}
