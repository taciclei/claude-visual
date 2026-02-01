//! Interactive rating component

use gpui::prelude::*;
use gpui::*;
use std::sync::Arc;

use super::types::*;
use crate::app::state::AppState;

/// Rating component
pub struct Rating {
    app_state: Arc<AppState>,
    /// Current rating value (0.0 to max)
    value: f32,
    /// Maximum rating value
    max: u8,
    /// Whether half ratings allowed
    allow_half: bool,
    /// Size variant
    size: RatingSize,
    /// Whether rating is readonly
    readonly: bool,
    /// Whether rating is disabled
    disabled: bool,
    /// Icon for filled state
    filled_icon: String,
    /// Icon for empty state
    empty_icon: String,
    /// Icon for half state
    half_icon: String,
    /// Hover value (when hovering)
    hover_value: Option<f32>,
    /// Label
    label: Option<String>,
    /// Show value
    show_value: bool,
}

impl Rating {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            value: 0.0,
            max: 5,
            allow_half: false,
            size: RatingSize::default(),
            readonly: false,
            disabled: false,
            filled_icon: "★".to_string(),
            empty_icon: "☆".to_string(),
            half_icon: "⯪".to_string(),
            hover_value: None,
            label: None,
            show_value: false,
        }
    }

    /// Set value
    pub fn set_value(&mut self, value: f32, cx: &mut Context<Self>) {
        let clamped = value.clamp(0.0, self.max as f32);
        let adjusted = if self.allow_half {
            (clamped * 2.0).round() / 2.0
        } else {
            clamped.round()
        };

        if (adjusted - self.value).abs() > f32::EPSILON {
            self.value = adjusted;
            cx.emit(RatingEvent::Changed(adjusted));
            cx.notify();
        }
    }

    /// Get value
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Set max
    pub fn set_max(&mut self, max: u8, cx: &mut Context<Self>) {
        self.max = max.max(1);
        if self.value > self.max as f32 {
            self.value = self.max as f32;
        }
        cx.notify();
    }

    /// Set allow half
    pub fn set_allow_half(&mut self, allow: bool, cx: &mut Context<Self>) {
        self.allow_half = allow;
        cx.notify();
    }

    /// Set size
    pub fn set_size(&mut self, size: RatingSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set readonly
    pub fn set_readonly(&mut self, readonly: bool, cx: &mut Context<Self>) {
        self.readonly = readonly;
        cx.notify();
    }

    /// Set disabled
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }

    /// Set icons
    pub fn set_icons(
        &mut self,
        filled: impl Into<String>,
        empty: impl Into<String>,
        half: impl Into<String>,
        cx: &mut Context<Self>,
    ) {
        self.filled_icon = filled.into();
        self.empty_icon = empty.into();
        self.half_icon = half.into();
        cx.notify();
    }

    /// Set label
    pub fn set_label(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.label = label;
        cx.notify();
    }

    /// Set show value
    pub fn set_show_value(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_value = show;
        cx.notify();
    }

    /// Clear rating
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.set_value(0.0, cx);
    }

    /// Handle click on a star
    fn handle_click(&mut self, index: u8, cx: &mut Context<Self>) {
        if self.readonly || self.disabled {
            return;
        }

        let new_value = if self.allow_half && self.value == index as f32 + 0.5 {
            index as f32 + 1.0
        } else if self.allow_half && self.value == index as f32 + 1.0 {
            index as f32 + 0.5
        } else {
            index as f32 + 1.0
        };

        self.set_value(new_value, cx);
    }

    /// Get display value (considering hover)
    fn display_value(&self) -> f32 {
        self.hover_value.unwrap_or(self.value)
    }
}

impl EventEmitter<RatingEvent> for Rating {}

impl Render for Rating {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let icon_size = self.size.icon_size();
        let opacity = if self.disabled { 0.5 } else { 1.0 };
        let display_value = self.display_value();

        let filled_color = hsla(0.12, 0.9, 0.5, 1.0); // Gold
        let empty_color = theme.colors.text_muted;

        div()
            .id("rating")
            .flex()
            .flex_col()
            .gap_1()
            .opacity(opacity)
            // Label
            .when_some(self.label.clone(), |d, label| {
                d.child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.colors.text)
                        .child(label),
                )
            })
            // Stars row
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    // Stars
                    .children((0..self.max).map(|i| {
                        let filled = display_value >= (i as f32 + 1.0);
                        let half =
                            !filled && display_value > i as f32 && display_value < (i as f32 + 1.0);

                        let icon = if filled {
                            self.filled_icon.clone()
                        } else if half {
                            self.half_icon.clone()
                        } else {
                            self.empty_icon.clone()
                        };

                        let color = if filled || half {
                            filled_color
                        } else {
                            empty_color
                        };

                        div()
                            .id(SharedString::from(format!("star-{}", i)))
                            .text_size(px(icon_size))
                            .text_color(color)
                            .when(!self.readonly && !self.disabled, |d| {
                                d.cursor_pointer()
                                    .hover(|s| s.text_color(filled_color))
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.handle_click(i, cx);
                                    }))
                            })
                            .child(icon)
                    }))
                    // Value display
                    .when(self.show_value, |d| {
                        d.child(
                            div()
                                .ml_2()
                                .text_sm()
                                .text_color(theme.colors.text_muted)
                                .child(format!("{:.1}", self.value)),
                        )
                    }),
            )
    }
}
