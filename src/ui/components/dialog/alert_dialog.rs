//! Alert dialog component (acknowledgment only)

use gpui::*;
use gpui::prelude::*;
use super::types::*;
use super::dialog::Dialog;

/// Alert dialog (acknowledgment only)
#[derive(Clone)]
pub struct AlertDialog {
    title: String,
    message: String,
    button_label: String,
    icon: String,
    alert_type: AlertType,
}

impl AlertDialog {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            button_label: "OK".to_string(),
            icon: AlertType::Info.icon().to_string(),
            alert_type: AlertType::Info,
        }
    }

    pub fn alert_type(mut self, alert_type: AlertType) -> Self {
        self.alert_type = alert_type;
        self.icon = alert_type.icon().to_string();
        self
    }

    pub fn button_label(mut self, label: impl Into<String>) -> Self {
        self.button_label = label.into();
        self
    }

    pub fn info(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message).alert_type(AlertType::Info)
    }

    pub fn success(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message).alert_type(AlertType::Success)
    }

    pub fn warning(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message).alert_type(AlertType::Warning)
    }

    pub fn error(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message).alert_type(AlertType::Error)
    }
}

impl RenderOnce for AlertDialog {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        Dialog::new(self.title)
            .description(self.message)
            .icon(self.icon)
            .size(DialogSize::Small)
            .hide_close()
            .button(DialogButton::primary("ok", self.button_label))
    }
}
