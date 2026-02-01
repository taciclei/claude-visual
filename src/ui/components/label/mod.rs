//! Form label and helper text components

use gpui::prelude::*;
use gpui::*;

mod character_count;
mod description_label;
mod field_group;
mod form_field;
mod helper_text;
mod inline_label;
mod label;
mod types;

pub use character_count::CharacterCount;
pub use description_label::DescriptionLabel;
pub use field_group::FieldGroup;
pub use form_field::FormField;
pub use helper_text::HelperText;
pub use inline_label::InlineLabel;
pub use label::Label;
pub use types::{HelperTextVariant, LabelSize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label() {
        let label = Label::new("Email").required(true).size(LabelSize::Medium);
        assert_eq!(label.text.as_ref(), "Email");
        assert!(label.required);
    }

    #[test]
    fn test_helper_text() {
        let helper = HelperText::error("This field is required");
        assert_eq!(helper.variant, HelperTextVariant::Error);
    }

    #[test]
    fn test_character_count() {
        let counter = CharacterCount::new(50, 100)
            .warning_threshold(80)
            .show_remaining(true);
        assert_eq!(counter.current, 50);
        assert_eq!(counter.max, 100);
        assert!(counter.show_remaining);
    }

    #[test]
    fn test_form_field() {
        let field = FormField::new()
            .label(Label::new("Username").required(true))
            .helper(HelperText::new("Enter a unique username"))
            .gap(8.0);
        assert!(field.label.is_some());
        assert!(field.helper.is_some());
    }

    #[test]
    fn test_description_label() {
        let dl = DescriptionLabel::new("Status", "Active").inline(true);
        assert_eq!(dl.term.as_ref(), "Status");
        assert_eq!(dl.description.as_ref(), "Active");
        assert!(dl.inline);
    }
}
