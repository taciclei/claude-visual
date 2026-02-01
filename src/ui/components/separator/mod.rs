//! Separator and visual divider components

mod decorative_separator;
mod gap;
mod labeled_separator;
mod section_separator;
mod separator;
mod spacer;
mod timeline_separator;
mod types;

pub use decorative_separator::DecorativeSeparator;
pub use gap::Gap;
pub use labeled_separator::LabeledSeparator;
pub use section_separator::SectionSeparator;
pub use separator::Separator;
pub use spacer::Spacer;
pub use timeline_separator::TimelineSeparator;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separator() {
        let sep = Separator::horizontal()
            .thickness(SeparatorThickness::Thick)
            .margin(16.0);
        assert_eq!(sep.orientation, SeparatorOrientation::Horizontal);
        assert_eq!(sep.margin, 16.0);
    }

    #[test]
    fn test_labeled_separator() {
        let sep = LabeledSeparator::new("OR").position(LabelPosition::Center);
        assert_eq!(sep.label.as_ref(), "OR");
        assert_eq!(sep.position, LabelPosition::Center);
    }

    #[test]
    fn test_decorative_separator() {
        let sep = DecorativeSeparator::stars().margin(20.0);
        assert_eq!(sep.pattern, SeparatorPattern::Stars);
        assert_eq!(sep.margin, 20.0);
    }

    #[test]
    fn test_timeline_separator() {
        let sep = TimelineSeparator::new().completed(true).length(50.0);
        assert!(sep.completed);
        assert_eq!(sep.length, 50.0);
    }

    #[test]
    fn test_spacer() {
        let spacer = Spacer::fixed(24.0);
        assert_eq!(spacer.size, Some(24.0));
        assert!(!spacer.flex);
    }

    #[test]
    fn test_gap() {
        let gap = Gap::new(32.0).show_line(true);
        assert_eq!(gap.size, 32.0);
        assert!(gap.show_line);
    }
}
