//! Switch/Toggle component module

mod types;
mod switch;
mod checkbox;
mod radio_button;
mod radio_group;

pub use types::*;
pub use switch::Switch;
pub use checkbox::Checkbox;
pub use radio_button::RadioButton;
pub use radio_group::RadioGroup;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_size() {
        assert_eq!(SwitchSize::Small.track_width(), 32.0);
        assert_eq!(SwitchSize::Medium.track_width(), 44.0);
        assert_eq!(SwitchSize::Large.track_width(), 56.0);
    }

    #[test]
    fn test_radio_group_option_builder() {
        let option = RadioGroupOption::new("test", "Test Option")
            .with_description("A test description")
            .disabled();

        assert_eq!(option.value, "test");
        assert_eq!(option.label, "Test Option");
        assert_eq!(option.description, Some("A test description".to_string()));
        assert!(option.disabled);
    }
}
