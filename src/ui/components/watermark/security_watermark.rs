use gpui::prelude::*;
use gpui::*;

/// Security watermark with user info
#[derive(IntoElement)]
pub struct SecurityWatermark {
    id: ElementId,
    user_id: SharedString,
    timestamp: Option<SharedString>,
    ip_address: Option<SharedString>,
    show_user_id: bool,
    show_timestamp: bool,
    show_ip: bool,
    opacity: f32,
    color: gpui::Hsla,
    tiled: bool,
}

impl SecurityWatermark {
    pub fn new(id: impl Into<ElementId>, user_id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            user_id: user_id.into(),
            timestamp: None,
            ip_address: None,
            show_user_id: true,
            show_timestamp: true,
            show_ip: false,
            opacity: 0.05,
            color: rgba(0x000000ff).into(),
            tiled: true,
        }
    }

    pub fn timestamp(mut self, timestamp: impl Into<SharedString>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    pub fn ip_address(mut self, ip: impl Into<SharedString>) -> Self {
        self.ip_address = Some(ip.into());
        self
    }

    pub fn show_user_id(mut self, show: bool) -> Self {
        self.show_user_id = show;
        self
    }

    pub fn show_timestamp(mut self, show: bool) -> Self {
        self.show_timestamp = show;
        self
    }

    pub fn show_ip(mut self, show: bool) -> Self {
        self.show_ip = show;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn color(mut self, color: gpui::Hsla) -> Self {
        self.color = color;
        self
    }

    pub fn tiled(mut self, tiled: bool) -> Self {
        self.tiled = tiled;
        self
    }

    pub(crate) fn build_text(&self) -> String {
        let mut parts = Vec::new();
        if self.show_user_id {
            parts.push(self.user_id.to_string());
        }
        if self.show_timestamp {
            if let Some(ref ts) = self.timestamp {
                parts.push(ts.to_string());
            }
        }
        if self.show_ip {
            if let Some(ref ip) = self.ip_address {
                parts.push(ip.to_string());
            }
        }
        parts.join(" | ")
    }
}

impl RenderOnce for SecurityWatermark {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = self.build_text();

        if self.tiled {
            div()
                .id(self.id)
                .absolute()
                .inset_0()
                .overflow_hidden()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(80.0))
                .children((0..5).map(|_row| {
                    div().flex().gap(px(120.0)).children((0..3).map(|_col| {
                        div()
                            .text_xs()
                            .text_color(self.color.opacity(self.opacity))
                            .child(text.clone())
                    }))
                }))
        } else {
            div()
                .id(self.id)
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_sm()
                        .text_color(self.color.opacity(self.opacity))
                        .child(text),
                )
        }
    }
}
