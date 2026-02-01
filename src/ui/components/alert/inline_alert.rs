//! Inline alert component (stateless)

use gpui::prelude::*;
use gpui::*;

use super::types::*;

/// A simple inline alert (stateless version)
#[derive(Clone)]
pub struct InlineAlert {
    message: String,
    alert_type: AlertType,
    icon: Option<String>,
}

impl InlineAlert {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            alert_type: AlertType::Info,
            icon: None,
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        let mut alert = Self::new(message);
        alert.alert_type = AlertType::Info;
        alert
    }

    pub fn success(message: impl Into<String>) -> Self {
        let mut alert = Self::new(message);
        alert.alert_type = AlertType::Success;
        alert
    }

    pub fn warning(message: impl Into<String>) -> Self {
        let mut alert = Self::new(message);
        alert.alert_type = AlertType::Warning;
        alert
    }

    pub fn error(message: impl Into<String>) -> Self {
        let mut alert = Self::new(message);
        alert.alert_type = AlertType::Error;
        alert
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl RenderOnce for InlineAlert {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (accent, bg) = match self.alert_type {
            AlertType::Info => (
                hsla(210.0 / 360.0, 0.8, 0.5, 1.0),
                hsla(210.0 / 360.0, 0.8, 0.5, 0.1),
            ),
            AlertType::Success => (
                hsla(145.0 / 360.0, 0.6, 0.4, 1.0),
                hsla(145.0 / 360.0, 0.6, 0.4, 0.1),
            ),
            AlertType::Warning => (
                hsla(45.0 / 360.0, 0.9, 0.5, 1.0),
                hsla(45.0 / 360.0, 0.9, 0.5, 0.1),
            ),
            AlertType::Error => (hsla(0.0, 0.7, 0.5, 1.0), hsla(0.0, 0.7, 0.5, 0.1)),
        };

        let icon = self
            .icon
            .unwrap_or_else(|| self.alert_type.icon().to_string());

        div()
            .px_3()
            .py_2()
            .rounded(px(6.0))
            .bg(bg)
            .flex()
            .items_center()
            .gap_2()
            .child(div().text_color(accent).child(icon))
            .child(div().text_sm().text_color(accent).child(self.message))
    }
}
