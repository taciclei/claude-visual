//! Select components - Dropdown selection with search and groups
//!
//! Provides select/combobox components for choosing from a list of options.

mod types;
mod select;
mod multi_select;
mod grouped_select;
mod native_select;
mod dropdown;

pub use types::{SelectSize, SelectVariant, SelectOption, SelectGroup};
pub use select::Select;
pub use multi_select::MultiSelect;
pub use grouped_select::GroupedSelect;
pub use native_select::NativeSelect;
pub use dropdown::SelectDropdown;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_option() {
        let option = SelectOption::new("val", "Label")
            .description("A description")
            .disabled(true)
            .icon("📌");

        assert_eq!(option.value.as_ref(), "val");
        assert_eq!(option.label.as_ref(), "Label");
        assert_eq!(option.description.unwrap().as_ref(), "A description");
        assert!(option.disabled);
        assert_eq!(option.icon.unwrap().as_ref(), "📌");
    }

    #[test]
    fn test_select_group() {
        let options = vec![
            SelectOption::new("a", "Option A"),
            SelectOption::new("b", "Option B"),
        ];
        let group = SelectGroup::new("Group 1", options);

        assert_eq!(group.label.as_ref(), "Group 1");
        assert_eq!(group.options.len(), 2);
    }

    #[test]
    fn test_select_sizes() {
        let small = Select::new("s").size(SelectSize::Small);
        let medium = Select::new("m").size(SelectSize::Medium);
        let large = Select::new("l").size(SelectSize::Large);

        assert_eq!(small.size, SelectSize::Small);
        assert_eq!(medium.size, SelectSize::Medium);
        assert_eq!(large.size, SelectSize::Large);
    }

    #[test]
    fn test_multi_select() {
        let selected = vec!["a".into(), "b".into()];
        let multi = MultiSelect::new("multi")
            .selected(selected.clone())
            .max_items(5);

        assert_eq!(multi.selected.len(), 2);
        assert_eq!(multi.max_items, Some(5));
    }
}
