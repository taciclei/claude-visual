//! Form components - Form containers, validation, and field groups
//!
//! Provides form layout and validation components.

mod actions;
mod field;
mod fieldset;
mod form;
mod row;
mod section;
mod types;

pub use actions::*;
pub use field::*;
pub use fieldset::*;
pub use form::*;
pub use row::*;
pub use section::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_field_state() {
        let state = FormFieldState::new("email")
            .value("test@example.com")
            .valid()
            .touched(true);

        assert_eq!(state.name.as_ref(), "email");
        assert_eq!(state.value.as_ref(), "test@example.com");
        assert_eq!(state.validation, ValidationState::Valid);
        assert!(state.touched);
    }

    #[test]
    fn test_form_field_invalid() {
        let state = FormFieldState::new("password").invalid("Password too short");

        assert_eq!(state.validation, ValidationState::Invalid);
        assert_eq!(state.error.unwrap().as_ref(), "Password too short");
    }

    #[test]
    fn test_form_layouts() {
        let vertical = Form::new("v").layout(FormLayout::Vertical);
        let horizontal = Form::new("h").layout(FormLayout::Horizontal);
        let inline = Form::new("i").layout(FormLayout::Inline);

        assert_eq!(vertical.layout, FormLayout::Vertical);
        assert_eq!(horizontal.layout, FormLayout::Horizontal);
        assert_eq!(inline.layout, FormLayout::Inline);
    }

    #[test]
    fn test_form_sizes() {
        let small = Form::new("s").size(FormSize::Small);
        let medium = Form::new("m").size(FormSize::Medium);
        let large = Form::new("l").size(FormSize::Large);

        assert_eq!(small.size, FormSize::Small);
        assert_eq!(medium.size, FormSize::Medium);
        assert_eq!(large.size, FormSize::Large);
    }

    #[test]
    fn test_form_actions_alignment() {
        let left = FormActions::new().alignment(FormActionsAlignment::Left);
        let center = FormActions::new().alignment(FormActionsAlignment::Center);
        let right = FormActions::new().alignment(FormActionsAlignment::Right);

        assert_eq!(left.alignment, FormActionsAlignment::Left);
        assert_eq!(center.alignment, FormActionsAlignment::Center);
        assert_eq!(right.alignment, FormActionsAlignment::Right);
    }

    #[test]
    fn test_validation_states() {
        let none = ValidationState::None;
        let valid = ValidationState::Valid;
        let invalid = ValidationState::Invalid;
        let warning = ValidationState::Warning;

        assert_eq!(none, ValidationState::None);
        assert_eq!(valid, ValidationState::Valid);
        assert_eq!(invalid, ValidationState::Invalid);
        assert_eq!(warning, ValidationState::Warning);
    }
}
