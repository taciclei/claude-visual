//! Tag/Chip component for labels and categories

mod tag;
mod tag_group;
mod types;

pub use tag::Tag;
pub use tag_group::TagGroup;
pub use types::{TagColor, TagEvent, TagGroupItem, TagSize};

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
