//! Core pagination types

/// Page information for cursor-based pagination
#[derive(Debug, Clone)]
pub struct PageInfo {
    /// Whether there are more items before the first item
    pub has_previous_page: bool,
    /// Whether there are more items after the last item
    pub has_next_page: bool,
    /// Cursor of the first item
    pub start_cursor: Option<String>,
    /// Cursor of the last item
    pub end_cursor: Option<String>,
    /// Total count (if known)
    pub total_count: Option<usize>,
}

impl PageInfo {
    /// Create empty page info
    pub fn empty() -> Self {
        Self {
            has_previous_page: false,
            has_next_page: false,
            start_cursor: None,
            end_cursor: None,
            total_count: Some(0),
        }
    }
}

/// Direction of pagination
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaginationDirection {
    /// Fetch items going forward (older to newer)
    Forward,
    /// Fetch items going backward (newer to older)
    Backward,
}

/// Paginated result
#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    /// The items in this page
    pub items: Vec<T>,
    /// Pagination information
    pub page_info: PageInfo,
}

impl<T> PaginatedResult<T> {
    /// Create a new paginated result
    pub fn new(items: Vec<T>, page_info: PageInfo) -> Self {
        Self { items, page_info }
    }

    /// Create an empty result
    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            page_info: PageInfo::empty(),
        }
    }

    /// Map items to a different type
    pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> PaginatedResult<U> {
        PaginatedResult {
            items: self.items.into_iter().map(f).collect(),
            page_info: self.page_info,
        }
    }
}
