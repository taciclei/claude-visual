//! Tag/Chip component for labels and categories

mod types;
mod tag;
mod tag_group;

pub use types::{TagColor, TagSize, TagEvent, TagGroupItem};
pub use tag::Tag;
pub use tag_group::TagGroup;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_size() {
        assert_eq!(TagSize::Small.height(), 20.0);
        assert_eq!(TagSize::Medium.height(), 24.0);
        assert_eq!(TagSize::Large.height(), 28.0);
    }

    #[test]
    fn test_tag_group_item_builder() {
        let item = TagGroupItem::new("Test")
            .with_color(TagColor::Success)
            .with_icon("✓");

        assert_eq!(item.label, "Test");
        assert_eq!(item.color, TagColor::Success);
        assert_eq!(item.icon, Some("✓".to_string()));
    }
}
