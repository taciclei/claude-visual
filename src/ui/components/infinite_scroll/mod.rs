//! Infinite scroll and lazy loading components
//!
//! Provides infinite scrolling, lazy loading, and pull-to-refresh functionality.

mod infinite_scroll;
mod lazy_load_container;
mod load_more_trigger;
mod pull_to_refresh;
mod types;
mod virtual_scroller;

pub use infinite_scroll::InfiniteScroll;
pub use lazy_load_container::LazyLoadContainer;
pub use load_more_trigger::LoadMoreTrigger;
pub use pull_to_refresh::PullToRefresh;
pub use types::*;
pub use virtual_scroller::VirtualScroller;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infinite_scroll_creation() {
        let scroll = InfiniteScroll::new("test-scroll")
            .direction(ScrollDirection::Down)
            .threshold(300.0)
            .items_per_page(50);
        assert_eq!(scroll.direction, ScrollDirection::Down);
        assert_eq!(scroll.threshold, 300.0);
        assert_eq!(scroll.items_per_page, 50);
    }

    #[test]
    fn test_lazy_load_container() {
        let container = LazyLoadContainer::new("lazy-1")
            .loaded(true)
            .placeholder_height(300.0);
        assert!(container.loaded);
        assert_eq!(container.placeholder_height, 300.0);
    }

    #[test]
    fn test_pull_to_refresh() {
        let ptr = PullToRefresh::new("ptr-1")
            .state(PullState::Pulling)
            .pull_distance(50.0)
            .threshold(100.0);
        assert_eq!(ptr.state, PullState::Pulling);
        assert_eq!(ptr.pull_distance, 50.0);
    }

    #[test]
    fn test_virtual_scroller() {
        let scroller = VirtualScroller::new("vs-1")
            .total_items(1000)
            .item_height(40.0)
            .visible_items(20)
            .scroll_offset(400.0);
        assert_eq!(scroller.total_height(), 40000.0);
        let (start, end) = scroller.visible_range();
        assert!(start <= 10); // start - overscan
        assert!(end >= 10 + 20); // start + visible
    }

    #[test]
    fn test_load_more_trigger() {
        let trigger = LoadMoreTrigger::new("lmt-1")
            .loading(true)
            .has_more(true)
            .variant(LoadMoreVariant::Link);
        assert!(trigger.loading);
        assert!(trigger.has_more);
        assert_eq!(trigger.variant, LoadMoreVariant::Link);
    }
}
