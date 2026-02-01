//! Dropdown/Select component for selection menus

mod types;
mod state;
mod methods;
mod render;
mod option_list;

pub use types::*;
pub use state::Dropdown;
pub use option_list::OptionList;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dropdown_size() {
        assert_eq!(DropdownSize::Small.height(), 28.0);
        assert_eq!(DropdownSize::Medium.height(), 36.0);
        assert_eq!(DropdownSize::Large.height(), 44.0);
    }

    #[test]
    fn test_dropdown_option_builder() {
        let option = DropdownOption::new("test", "Test Option")
            .with_description("A test description")
            .with_icon("🔧")
            .disabled();

        assert_eq!(option.id, "test");
        assert_eq!(option.label, "Test Option");
        assert_eq!(option.description, Some("A test description".to_string()));
        assert_eq!(option.icon, Some("🔧".to_string()));
        assert!(option.disabled);
    }
}
