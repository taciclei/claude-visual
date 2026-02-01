//! Pagination request types

use super::{Cursor, PaginationDirection};

/// Pagination request parameters
#[derive(Debug, Clone)]
pub struct PaginationRequest {
    /// Number of items to fetch
    pub limit: usize,
    /// Cursor for the starting point
    pub cursor: Option<Cursor>,
    /// Direction of pagination
    pub direction: PaginationDirection,
}

impl PaginationRequest {
    /// Create a request for the first page
    pub fn first(limit: usize) -> Self {
        Self {
            limit,
            cursor: None,
            direction: PaginationDirection::Forward,
        }
    }

    /// Create a request for the last page (most recent)
    pub fn last(limit: usize) -> Self {
        Self {
            limit,
            cursor: None,
            direction: PaginationDirection::Backward,
        }
    }

    /// Create a request for items after a cursor
    pub fn after(cursor: String, limit: usize) -> Self {
        Self {
            limit,
            cursor: Some(Cursor::After(cursor)),
            direction: PaginationDirection::Forward,
        }
    }

    /// Create a request for items before a cursor
    pub fn before(cursor: String, limit: usize) -> Self {
        Self {
            limit,
            cursor: Some(Cursor::Before(cursor)),
            direction: PaginationDirection::Backward,
        }
    }
}

impl Default for PaginationRequest {
    fn default() -> Self {
        Self::last(50) // Default to most recent 50 messages
    }
}
