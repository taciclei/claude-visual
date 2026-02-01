//! Combobox components - Input with dropdown autocomplete
//!
//! Provides combobox components that combine text input with dropdown selection.

mod types;
mod combobox;
mod combobox_dropdown;
mod tag_input;
mod autocomplete;

pub use types::*;
pub use combobox::Combobox;
pub use combobox_dropdown::ComboboxDropdown;
pub use tag_input::TagInput;
pub use autocomplete::Autocomplete;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combobox_item() {
        let item = ComboboxItem::new("val", "Label")
            .description("Description")
            .icon("🔍")
            .disabled(true);

        assert_eq!(item.value.as_ref(), "val");
        assert_eq!(item.label.as_ref(), "Label");
        assert!(item.disabled);
    }

    #[test]
    fn test_combobox_sizes() {
        let small = Combobox::new("s").size(ComboboxSize::Small);
        let medium = Combobox::new("m").size(ComboboxSize::Medium);
        let large = Combobox::new("l").size(ComboboxSize::Large);

        assert_eq!(small.size, ComboboxSize::Small);
        assert_eq!(medium.size, ComboboxSize::Medium);
        assert_eq!(large.size, ComboboxSize::Large);
    }

    #[test]
    fn test_combobox_modes() {
        let filter = Combobox::new("f").mode(ComboboxMode::Filter);
        let search = Combobox::new("s").mode(ComboboxMode::Search);
        let create = Combobox::new("c").mode(ComboboxMode::Create);

        assert_eq!(filter.mode, ComboboxMode::Filter);
        assert_eq!(search.mode, ComboboxMode::Search);
        assert_eq!(create.mode, ComboboxMode::Create);
    }

    #[test]
    fn test_tag_input() {
        let selected = vec!["a".into(), "b".into()];
        let tag_input = TagInput::new("tags")
            .selected(selected.clone())
            .max_tags(5)
            .allow_create(true);

        assert_eq!(tag_input.selected.len(), 2);
        assert_eq!(tag_input.max_tags, Some(5));
        assert!(tag_input.allow_create);
    }
}
