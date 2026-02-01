//! Trait implementations for UpdateNotification

use gpui::*;
use gpui::prelude::*;

use crate::update::UpdateStatus;
use super::core::UpdateNotification;
use super::types::{default_colors, UpdateNotificationEvent};

impl Render for UpdateNotification {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = default_colors();

        if !self.is_visible() {
            return div().into_any_element();
        }

        // If we're in the middle of installation, show progress
        if let Some(ref progress) = self.install_progress {
            return div()
                .w_full()
                .p_4()
                .child(self.render_progress(progress, &theme))
                .into_any_element();
        }

        match &self.update_status {
            UpdateStatus::UpdateAvailable(info) => {
                let info = info.clone();
                div()
                    .w_full()
                    .p_4()
                    .child(self.render_banner(&info, &theme, cx))
                    .into_any_element()
            }
            UpdateStatus::Error(error) => {
                let error = error.clone();
                div()
                    .w_full()
                    .p_4()
                    .child(self.render_error(&error, &theme, cx))
                    .into_any_element()
            }
            _ => div().into_any_element(),
        }
    }
}

impl EventEmitter<UpdateNotificationEvent> for UpdateNotification {}

impl Focusable for UpdateNotification {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
