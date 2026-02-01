//! Simple notification banner component

use gpui::prelude::*;
use gpui::*;

use super::types::*;

/// Simple notification banner
#[derive(Clone)]
pub struct NotificationBanner {
    pub(crate) message: String,
    pub(crate) banner_type: BannerType,
    pub(crate) dismissible: bool,
}

impl NotificationBanner {
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            banner_type: BannerType::Info,
            dismissible: true,
        }
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            banner_type: BannerType::Success,
            dismissible: true,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            banner_type: BannerType::Warning,
            dismissible: true,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            banner_type: BannerType::Error,
            dismissible: true,
        }
    }

    pub fn not_dismissible(mut self) -> Self {
        self.dismissible = false;
        self
    }
}

impl RenderOnce for NotificationBanner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        let (bg_color, icon_color, icon) = match self.banner_type {
            BannerType::Info => (hsla(0.6, 0.8, 0.5, 0.15), hsla(0.6, 0.8, 0.6, 1.0), "ℹ️"),
            BannerType::Success => (hsla(0.38, 0.7, 0.45, 0.15), hsla(0.38, 0.7, 0.45, 1.0), "✓"),
            BannerType::Warning => (hsla(0.12, 0.9, 0.5, 0.15), hsla(0.12, 0.9, 0.5, 1.0), "⚠️"),
            BannerType::Error => (hsla(0.0, 0.7, 0.5, 0.15), hsla(0.0, 0.7, 0.5, 1.0), "✕"),
            BannerType::Announcement => {
                (hsla(0.75, 0.7, 0.5, 0.15), hsla(0.75, 0.7, 0.5, 1.0), "📢")
            }
        };

        div()
            .w_full()
            .px_4()
            .py_3()
            .bg(bg_color)
            .flex()
            .items_center()
            .gap_3()
            .child(div().text_base().text_color(icon_color).child(icon))
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(text)
                    .child(self.message),
            )
            .when(self.dismissible, |d| {
                d.child(
                    div()
                        .size(px(24.0))
                        .rounded(px(4.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_sm()
                        .text_color(text_muted)
                        .cursor_pointer()
                        .child("×"),
                )
            })
    }
}
