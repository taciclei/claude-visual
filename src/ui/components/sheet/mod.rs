//! Sheet component (bottom sheet, action sheet)

mod types;
mod sheet;
mod action_sheet;
mod confirm_sheet;
mod share_sheet;

// Re-export all public items
pub use types::{SheetPosition, SheetSize, SheetEvent, SheetAction, ShareItem};
pub use sheet::Sheet;
pub use action_sheet::ActionSheet;
pub use confirm_sheet::ConfirmSheet;
pub use share_sheet::ShareSheet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheet() {
        let sheet = Sheet::new()
            .position(SheetPosition::Bottom)
            .size(SheetSize::Medium)
            .title("My Sheet");

        assert_eq!(sheet.position, SheetPosition::Bottom);
        assert!(sheet.title.is_some());
    }

    #[test]
    fn test_sheet_size() {
        assert_eq!(SheetSize::Small.percentage(), 0.3);
        assert_eq!(SheetSize::Medium.percentage(), 0.5);
        assert_eq!(SheetSize::Large.percentage(), 0.75);
        assert_eq!(SheetSize::Custom(0.6).percentage(), 0.6);
    }

    #[test]
    fn test_action_sheet() {
        let sheet = ActionSheet::new()
            .title("Options")
            .action(SheetAction::new("Edit"))
            .action(SheetAction::new("Delete").destructive());

        assert_eq!(sheet.actions.len(), 2);
        assert!(sheet.actions[1].destructive);
    }

    #[test]
    fn test_confirm_sheet() {
        let sheet = ConfirmSheet::new("Delete?", "This cannot be undone.")
            .confirm_label("Delete")
            .destructive();

        assert_eq!(sheet.title, "Delete?");
        assert!(sheet.destructive);
    }
}
