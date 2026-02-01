//! Pagination component for navigating through pages

mod component;
mod page_size_selector;
mod render;
mod simple;
mod types;

pub use component::Pagination;
pub use page_size_selector::PageSizeSelector;
pub use simple::SimplePagination;
pub use types::{PaginationEvent, PaginationSize, PaginationStyle};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_pagination() {
        let pager = SimplePagination::new(5, 10);
        assert_eq!(pager.current, 5);
        assert_eq!(pager.total, 10);
    }

    #[test]
    fn test_page_size_selector() {
        let selector = PageSizeSelector::new(25).options(vec![10, 20, 50]);

        assert_eq!(selector.current, 25);
        assert_eq!(selector.options, vec![10, 20, 50]);
    }
}
