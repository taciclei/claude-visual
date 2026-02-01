//! Main Alert component

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// Alert/Banner component
pub struct Alert {
    app_state: Arc<AppState>,
    /// Alert type/severity
    alert_type: AlertType,
    /// Style variant
    style: AlertStyle,
    /// Title (optional)
    title: Option<String>,
    /// Message/description
    message: String,
    /// Whether alert can be dismissed
    dismissible: bool,
    /// Custom icon (overrides default)
    custom_icon: Option<String>,
    /// Action button label
    action_label: Option<String>,
    /// Whether to show icon
    show_icon: bool,
}

impl Alert {
    pub fn new(
        app_state: Arc<AppState>,
        message: impl Into<String>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            alert_type: AlertType::default(),
            style: AlertStyle::default(),
            title: None,
            message: message.into(),
            dismissible: false,
            custom_icon: None,
            action_label: None,
            show_icon: true,
        }
    }

    /// Create an info alert
    pub fn info(
        app_state: Arc<AppState>,
        message: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut alert = Self::new(app_state, message, cx);
        alert.alert_type = AlertType::Info;
        alert
    }

    /// Create a success alert
    pub fn success(
        app_state: Arc<AppState>,
        message: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut alert = Self::new(app_state, message, cx);
        alert.alert_type = AlertType::Success;
        alert
    }

    /// Create a warning alert
    pub fn warning(
        app_state: Arc<AppState>,
        message: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut alert = Self::new(app_state, message, cx);
        alert.alert_type = AlertType::Warning;
        alert
    }

    /// Create an error alert
    pub fn error(
        app_state: Arc<AppState>,
        message: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut alert = Self::new(app_state, message, cx);
        alert.alert_type = AlertType::Error;
        alert
    }

    /// Set the alert type
    pub fn set_type(&mut self, alert_type: AlertType, cx: &mut Context<Self>) {
        self.alert_type = alert_type;
        cx.notify();
    }

    /// Set the style
    pub fn set_style(&mut self, style: AlertStyle, cx: &mut Context<Self>) {
        self.style = style;
        cx.notify();
    }

    /// Set the title
    pub fn set_title(&mut self, title: Option<String>, cx: &mut Context<Self>) {
        self.title = title;
        cx.notify();
    }

    /// Set the message
    pub fn set_message(&mut self, message: impl Into<String>, cx: &mut Context<Self>) {
        self.message = message.into();
        cx.notify();
    }

    /// Set dismissible
    pub fn set_dismissible(&mut self, dismissible: bool, cx: &mut Context<Self>) {
        self.dismissible = dismissible;
        cx.notify();
    }

    /// Set custom icon
    pub fn set_icon(&mut self, icon: Option<String>, cx: &mut Context<Self>) {
        self.custom_icon = icon;
        cx.notify();
    }

    /// Set action button
    pub fn set_action(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.action_label = label;
        cx.notify();
    }

    /// Show/hide icon
    pub fn set_show_icon(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_icon = show;
        cx.notify();
    }

    /// Get colors for the current type
    fn get_colors(&self, theme: &crate::app::theme::Theme) -> (Hsla, Hsla, Hsla) {
        let (accent, bg_opacity) = match self.alert_type {
            AlertType::Info => (hsla(210.0 / 360.0, 0.8, 0.5, 1.0), 0.1),
            AlertType::Success => (theme.colors.success, 0.1),
            AlertType::Warning => (theme.colors.warning, 0.1),
            AlertType::Error => (theme.colors.error, 0.1),
        };

        let bg = accent.opacity(bg_opacity);
        let border = accent.opacity(0.3);

        (accent, bg, border)
    }
}

impl EventEmitter<AlertEvent> for Alert {}

impl Render for Alert {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let (accent_color, bg_color, border_color) = self.get_colors(theme);
        let icon = self
            .custom_icon
            .clone()
            .unwrap_or_else(|| self.alert_type.icon().to_string());

        let (use_bg, use_border, use_left_accent) = match self.style {
            AlertStyle::Filled => (true, false, false),
            AlertStyle::Outline => (false, true, false),
            AlertStyle::Subtle => (true, true, false),
            AlertStyle::LeftAccent => (true, false, true),
        };

        div()
            .id("alert")
            .w_full()
            .p_3()
            .rounded_lg()
            .when(use_bg, |d| d.bg(bg_color))
            .when(use_border, |d| d.border_1().border_color(border_color))
            .when(use_left_accent, |d| {
                d.border_l_4().border_color(accent_color).rounded_l_none()
            })
            .flex()
            .items_start()
            .gap_3()
            // Icon
            .when(self.show_icon, |d| {
                d.child(
                    div()
                        .flex_shrink_0()
                        .size(px(20.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(accent_color)
                        .child(icon),
                )
            })
            // Content
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_1()
                    // Title
                    .when_some(self.title.clone(), |d, title| {
                        d.child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.colors.text)
                                .child(title),
                        )
                    })
                    // Message
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child(self.message.clone()),
                    )
                    // Action button
                    .when_some(self.action_label.clone(), |d, label| {
                        d.child(
                            div()
                                .id("alert-action")
                                .mt_2()
                                .px_3()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(accent_color.opacity(0.15))
                                .text_sm()
                                .text_color(accent_color)
                                .font_weight(FontWeight::MEDIUM)
                                .cursor_pointer()
                                .hover(|s| s.bg(accent_color.opacity(0.25)))
                                .on_click(cx.listener(|_this, _, _window, cx| {
                                    cx.emit(AlertEvent::ActionClicked);
                                }))
                                .child(label),
                        )
                    }),
            )
            // Dismiss button
            .when(self.dismissible, |d| {
                d.child(
                    div()
                        .id("alert-dismiss")
                        .flex_shrink_0()
                        .size(px(24.0))
                        .rounded(px(4.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(theme.colors.text_muted)
                        .cursor_pointer()
                        .hover(|s| {
                            s.bg(theme.colors.surface_hover)
                                .text_color(theme.colors.text)
                        })
                        .on_click(cx.listener(|_this, _, _window, cx| {
                            cx.emit(AlertEvent::Dismissed);
                        }))
                        .child("×"),
                )
            })
    }
}
