//! Dialog components for confirmations and alerts

mod types;
mod dialog;
mod confirm_dialog;
mod alert_dialog;
mod input_dialog;

// Re-export all public items
pub use types::{
    DialogSize,
    DialogButtonStyle,
    DialogButton,
    DialogEvent,
    AlertType,
};

pub use dialog::Dialog;
pub use confirm_dialog::ConfirmDialog;
pub use alert_dialog::AlertDialog;
pub use input_dialog::InputDialog;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_button() {
        let btn = DialogButton::primary("save", "Save Changes")
            .loading();

        assert_eq!(btn.id, "save");
        assert!(btn.loading);
    }

    #[test]
    fn test_dialog() {
        let dialog = Dialog::new("Confirm Action")
            .description("Are you sure?")
            .icon("⚠️")
            .size(DialogSize::Small)
            .button(DialogButton::secondary("cancel", "Cancel"))
            .button(DialogButton::primary("confirm", "Confirm"));

        assert_eq!(dialog.title, "Confirm Action");
        assert_eq!(dialog.buttons.len(), 2);
    }

    #[test]
    fn test_confirm_dialog() {
        let dialog = ConfirmDialog::new("Delete?", "This cannot be undone.")
            .confirm_label("Delete")
            .destructive();

        assert!(dialog.destructive);
    }

    #[test]
    fn test_alert_dialog() {
        let dialog = AlertDialog::error("Error", "Something went wrong");
        assert!(matches!(dialog.alert_type, AlertType::Error));
    }

    #[test]
    fn test_input_dialog() {
        let dialog = InputDialog::new("Enter Name")
            .placeholder("Your name...")
            .default_value("John");

        assert_eq!(dialog.default_value, "John");
    }
}
