//! ConfirmSheet component

use gpui::*;
use gpui::prelude::*;
use super::types::*;
use super::action_sheet::ActionSheet;

/// Confirmation sheet
#[derive(Clone)]
pub struct ConfirmSheet {
    pub(crate) title: String,
    message: String,
    confirm_label: String,
    cancel_label: String,
    pub(crate) destructive: bool,
}

impl ConfirmSheet {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            confirm_label: "Confirm".to_string(),
            cancel_label: "Cancel".to_string(),
            destructive: false,
        }
    }

    pub fn confirm_label(mut self, label: impl Into<String>) -> Self {
        self.confirm_label = label.into();
        self
    }

    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }

    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}

impl RenderOnce for ConfirmSheet {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        ActionSheet::new()
            .title(self.title)
            .message(self.message)
            .action(
                if self.destructive {
                    SheetAction::new(self.confirm_label).destructive()
                } else {
                    SheetAction::new(self.confirm_label)
                }
            )
            .cancel_label(self.cancel_label)
    }
}
