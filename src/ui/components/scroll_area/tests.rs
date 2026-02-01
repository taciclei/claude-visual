//! Tests for scroll area components

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_scroll_area() {
        let area = ScrollArea::new()
            .width(300.0)
            .height(400.0)
            .direction(ScrollDirection::Vertical)
            .scrollbar_size(ScrollbarSize::Thin);
        assert_eq!(area.width, Some(300.0));
        assert_eq!(area.height, Some(400.0));
        assert_eq!(area.direction, ScrollDirection::Vertical);
    }

    #[test]
    fn test_scroll_indicator() {
        let indicator = ScrollIndicator::new()
            .position(0.5)
            .length(200.0)
            .visible_ratio(0.3);
        assert_eq!(indicator.position, 0.5);
        assert_eq!(indicator.length, 200.0);
        assert_eq!(indicator.visible_ratio, 0.3);
    }

    #[test]
    fn test_infinite_scroll() {
        let scroll = InfiniteScroll::new()
            .loading(true)
            .has_more(true)
            .height(500.0);
        assert!(scroll.loading);
        assert!(scroll.has_more);
        assert_eq!(scroll.height, 500.0);
    }

    #[test]
    fn test_pull_to_refresh() {
        let pull = PullToRefresh::new()
            .pulling(true)
            .pull_distance(50.0)
            .threshold(60.0);
        assert!(pull.pulling);
        assert_eq!(pull.pull_distance, 50.0);
        assert_eq!(pull.threshold, 60.0);
    }

    #[test]
    fn test_scroll_anchor() {
        let anchor = ScrollAnchor::new("section-1", "Introduction")
            .active(true);
        assert_eq!(anchor.id.as_ref(), "section-1");
        assert_eq!(anchor.label.as_ref(), "Introduction");
        assert!(anchor.active);
    }
}
