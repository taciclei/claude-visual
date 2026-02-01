//! Segmented control components
//!
//! Provides iOS-style segmented controls for switching between views.

mod button_group;
mod icon_segmented_control;
mod segmented_control;
mod types;
mod view_switcher;

pub use button_group::ButtonGroup;
pub use icon_segmented_control::IconSegmentedControl;
pub use segmented_control::SegmentedControl;
pub use types::{Segment, SegmentedSize, SegmentedVariant};
pub use view_switcher::ViewSwitcher;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segmented_sizes() {
        let xs = SegmentedSize::Xs;
        let lg = SegmentedSize::Lg;

        assert!(xs.height() < lg.height());
        assert!(xs.font_size() < lg.font_size());
    }

    #[test]
    fn test_segmented_control() {
        let segments = vec![
            Segment::new("day", "Day"),
            Segment::new("week", "Week"),
            Segment::new("month", "Month"),
        ];

        let control = SegmentedControl::new("sc")
            .segments(segments)
            .selected("week")
            .variant(SegmentedVariant::Filled);

        assert_eq!(control.segments.len(), 3);
        assert_eq!(control.selected.as_ref().map(|s| s.as_ref()), Some("week"));
    }

    #[test]
    fn test_segment_with_badge() {
        let segment = Segment::new("inbox", "Inbox").icon("📥").badge("5");

        assert!(segment.icon.is_some());
        assert!(segment.badge.is_some());
    }

    #[test]
    fn test_icon_segmented_control() {
        let control = IconSegmentedControl::new("isc")
            .icons(vec![("grid", "▦"), ("list", "☰"), ("cards", "▣")])
            .selected("list");

        assert_eq!(control.icons.len(), 3);
        assert_eq!(control.selected.as_ref().map(|s| s.as_ref()), Some("list"));
    }

    #[test]
    fn test_button_group() {
        let group = ButtonGroup::new("bg")
            .buttons(vec![
                ("left", "Left"),
                ("center", "Center"),
                ("right", "Right"),
            ])
            .attached(true);

        assert_eq!(group.buttons.len(), 3);
        assert!(group.attached);
    }

    #[test]
    fn test_view_switcher() {
        let switcher = ViewSwitcher::new("vs")
            .views(vec![
                ("all", "All", Some("📋")),
                ("active", "Active", None::<&str>),
                ("completed", "Completed", None::<&str>),
            ])
            .selected("active")
            .show_indicator(true);

        assert_eq!(switcher.views.len(), 3);
        assert!(switcher.show_indicator);
    }
}
