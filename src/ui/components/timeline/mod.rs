//! Timeline component for displaying events chronologically

mod types;
mod timeline;
mod simple_timeline;
mod activity_feed;

pub use types::*;
pub use timeline::Timeline;
pub use simple_timeline::SimpleTimeline;
pub use activity_feed::{ActivityFeed, ActivityItem};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_item() {
        let item = TimelineItem::new("Task completed")
            .description("All tests passed")
            .timestamp("10:30 AM")
            .completed();

        assert_eq!(item.title, "Task completed");
        assert_eq!(item.status, TimelineItemStatus::Completed);
    }

    #[test]
    fn test_simple_timeline() {
        let timeline = SimpleTimeline::new()
            .item("Started", "9:00 AM", true)
            .item("In Progress", "10:00 AM", false);

        assert_eq!(timeline.items.len(), 2);
    }

    #[test]
    fn test_activity_feed() {
        let feed = ActivityFeed::new()
            .item(
                ActivityItem::new("👤", "Alice", "commented on", "2h ago")
                    .target("Issue #123")
            );

        assert_eq!(feed.items.len(), 1);
    }
}
