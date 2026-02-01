//! Connection status rendering

use gpui::*;
use crate::app::theme::Theme;
use crate::ui::chat::view::types::ConnectionStatus;

/// Get connection status display data (color, icon, text)
pub(super) fn get_connection_status_data(
    status: &ConnectionStatus,
    theme: &Theme,
) -> (Hsla, &'static str, &'static str) {
    match status {
        ConnectionStatus::Disconnected => (theme.colors.text_muted, "○", "Disconnected"),
        ConnectionStatus::Connecting => (theme.colors.warning, "◐", "Connecting..."),
        ConnectionStatus::Connected => (theme.colors.success, "●", "Connected"),
        ConnectionStatus::Error => (theme.colors.error, "✕", "Error"),
    }
}

/// Render the connection status indicator
pub(super) fn render_connection_status(
    status: &ConnectionStatus,
    theme: &Theme,
) -> Div {
    let (status_color, status_icon, status_text) = get_connection_status_data(status, theme);

    div()
        .flex()
        .items_center()
        .gap_1()
        .child(
            div()
                .text_sm()
                .text_color(status_color)
                .child(status_icon)
        )
        .child(
            div()
                .text_xs()
                .text_color(status_color)
                .child(status_text)
        )
}
