//! Main Banner component

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// Banner component
pub struct Banner {
    app_state: Arc<AppState>,
    /// Banner message
    message: String,
    /// Banner type
    banner_type: BannerType,
    /// Position
    position: BannerPosition,
    /// Whether dismissible
    dismissible: bool,
    /// Whether banner is visible
    visible: bool,
    /// Action button label
    action_label: Option<String>,
    /// Action button ID
    action_id: Option<String>,
    /// Link text
    link_text: Option<String>,
    /// Link URL/ID
    link_id: Option<String>,
    /// Custom icon
    icon: Option<String>,
    /// Whether to show icon
    show_icon: bool,
}

impl Banner {
    pub fn new(
        app_state: Arc<AppState>,
        message: impl Into<String>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            message: message.into(),
            banner_type: BannerType::default(),
            position: BannerPosition::default(),
            dismissible: true,
            visible: true,
            action_label: None,
            action_id: None,
            link_text: None,
            link_id: None,
            icon: None,
            show_icon: true,
        }
    }

    /// Set message
    pub fn set_message(&mut self, message: impl Into<String>, cx: &mut Context<Self>) {
        self.message = message.into();
        cx.notify();
    }

    /// Set banner type
    pub fn set_type(&mut self, banner_type: BannerType, cx: &mut Context<Self>) {
        self.banner_type = banner_type;
        cx.notify();
    }

    /// Set position
    pub fn set_position(&mut self, position: BannerPosition, cx: &mut Context<Self>) {
        self.position = position;
        cx.notify();
    }

    /// Set dismissible
    pub fn set_dismissible(&mut self, dismissible: bool, cx: &mut Context<Self>) {
        self.dismissible = dismissible;
        cx.notify();
    }

    /// Set action
    pub fn set_action(
        &mut self,
        label: impl Into<String>,
        id: impl Into<String>,
        cx: &mut Context<Self>,
    ) {
        self.action_label = Some(label.into());
        self.action_id = Some(id.into());
        cx.notify();
    }

    /// Set link
    pub fn set_link(
        &mut self,
        text: impl Into<String>,
        id: impl Into<String>,
        cx: &mut Context<Self>,
    ) {
        self.link_text = Some(text.into());
        self.link_id = Some(id.into());
        cx.notify();
    }

    /// Set custom icon
    pub fn set_icon(&mut self, icon: Option<String>, cx: &mut Context<Self>) {
        self.icon = icon;
        cx.notify();
    }

    /// Set show icon
    pub fn set_show_icon(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_icon = show;
        cx.notify();
    }

    /// Show banner
    pub fn show(&mut self, cx: &mut Context<Self>) {
        self.visible = true;
        cx.notify();
    }

    /// Hide/dismiss banner
    pub fn dismiss(&mut self, cx: &mut Context<Self>) {
        self.visible = false;
        cx.emit(BannerEvent::Dismissed);
        cx.notify();
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl EventEmitter<BannerEvent> for Banner {}

impl Render for Banner {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.visible {
            return div().into_any_element();
        }

        let theme = self.app_state.theme.read(cx);

        let (bg_color, text_color, icon_color) = match self.banner_type {
            BannerType::Info => (
                theme.colors.accent.opacity(0.15),
                theme.colors.text,
                theme.colors.accent,
            ),
            BannerType::Success => (
                hsla(0.38, 0.7, 0.45, 0.15),
                theme.colors.text,
                hsla(0.38, 0.7, 0.45, 1.0),
            ),
            BannerType::Warning => (
                hsla(0.12, 0.9, 0.5, 0.15),
                theme.colors.text,
                hsla(0.12, 0.9, 0.5, 1.0),
            ),
            BannerType::Error => (
                theme.colors.error.opacity(0.15),
                theme.colors.text,
                theme.colors.error,
            ),
            BannerType::Announcement => (
                hsla(0.75, 0.7, 0.5, 0.15),
                theme.colors.text,
                hsla(0.75, 0.7, 0.5, 1.0),
            ),
        };

        let icon = self
            .icon
            .clone()
            .unwrap_or_else(|| self.banner_type.icon().to_string());

        div()
            .id("banner")
            .w_full()
            .px_4()
            .py_3()
            .bg(bg_color)
            .flex()
            .items_center()
            .gap_3()
            // Icon
            .when(self.show_icon, |d| {
                d.child(
                    div()
                        .flex_shrink_0()
                        .text_base()
                        .text_color(icon_color)
                        .child(icon),
                )
            })
            // Message
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(text_color)
                    .child(self.message.clone()),
            )
            // Link
            .when_some(self.link_text.clone(), |d, text| {
                let link_id = self.link_id.clone().unwrap_or_default();
                d.child(
                    div()
                        .id("banner-link")
                        .text_sm()
                        .text_color(theme.colors.accent)
                        .cursor_pointer()
                        .hover(|s| s.underline())
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            cx.emit(BannerEvent::LinkClicked(link_id.clone()));
                        }))
                        .child(text),
                )
            })
            // Action button
            .when_some(self.action_label.clone(), |d, label| {
                let action_id = self.action_id.clone().unwrap_or_default();
                d.child(
                    div()
                        .id("banner-action")
                        .px_3()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(icon_color)
                        .text_sm()
                        .text_color(gpui::white())
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.9))
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            cx.emit(BannerEvent::ActionClicked(action_id.clone()));
                        }))
                        .child(label),
                )
            })
            // Dismiss button
            .when(self.dismissible, |d| {
                d.child(
                    div()
                        .id("banner-dismiss")
                        .size(px(24.0))
                        .rounded(px(4.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_sm()
                        .text_color(theme.colors.text_muted)
                        .cursor_pointer()
                        .hover(|s| {
                            s.bg(theme.colors.surface_hover)
                                .text_color(theme.colors.text)
                        })
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.dismiss(cx);
                        }))
                        .child("×"),
                )
            })
            .into_any_element()
    }
}
