//! Confirm dialog component (yes/no)

use super::dialog::Dialog;
use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Confirm dialog (yes/no)
#[derive(Clone)]
pub struct ConfirmDialog {
    title: String,
    message: String,
    confirm_label: String,
    cancel_label: String,
    destructive: bool,
    icon: Option<String>,
}

impl ConfirmDialog {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            confirm_label: "Confirm".to_string(),
            cancel_label: "Cancel".to_string(),
            destructive: false,
            icon: None,
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

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl RenderOnce for ConfirmDialog {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut dialog = Dialog::new(self.title)
            .description(self.message)
            .size(DialogSize::Small)
            .button(DialogButton::secondary("cancel", self.cancel_label));

        if let Some(icon) = self.icon {
            dialog = dialog.icon(icon);
        }

        if self.destructive {
            dialog = dialog.button(DialogButton::destructive("confirm", self.confirm_label));
        } else {
            dialog = dialog.button(DialogButton::primary("confirm", self.confirm_label));
        }

        dialog
    }
}
