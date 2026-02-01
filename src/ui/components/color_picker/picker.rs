//! Main color picker component

use gpui::prelude::*;
use gpui::*;
use std::sync::Arc;

use super::types::*;
use crate::app::state::AppState;

/// Color picker component
pub struct ColorPicker {
    pub(crate) app_state: Arc<AppState>,
    /// Current color
    pub(crate) color: Hsla,
    /// Whether picker is open
    pub(crate) is_open: bool,
    /// Show alpha channel
    pub(crate) show_alpha: bool,
    /// Preset colors
    pub(crate) presets: Vec<Hsla>,
    /// Label
    pub(crate) label: Option<String>,
    /// Whether disabled
    pub(crate) disabled: bool,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}

impl ColorPicker {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            color: hsla(0.6, 0.8, 0.5, 1.0), // Blue default
            is_open: false,
            show_alpha: false,
            presets: Self::default_presets(),
            label: None,
            disabled: false,
            focus_handle: cx.focus_handle(),
        }
    }

    fn default_presets() -> Vec<Hsla> {
        vec![
            hsla(0.0, 0.0, 0.0, 1.0),  // Black
            hsla(0.0, 0.0, 0.5, 1.0),  // Gray
            hsla(0.0, 0.0, 1.0, 1.0),  // White
            hsla(0.0, 0.8, 0.5, 1.0),  // Red
            hsla(0.08, 0.8, 0.5, 1.0), // Orange
            hsla(0.15, 0.8, 0.5, 1.0), // Yellow
            hsla(0.33, 0.8, 0.4, 1.0), // Green
            hsla(0.55, 0.8, 0.5, 1.0), // Blue
            hsla(0.75, 0.8, 0.5, 1.0), // Purple
            hsla(0.9, 0.8, 0.5, 1.0),  // Pink
        ]
    }

    /// Set color
    pub fn set_color(&mut self, color: Hsla, cx: &mut Context<Self>) {
        self.color = color;
        cx.emit(ColorPickerEvent::Changed(color));
        cx.notify();
    }

    /// Get color
    pub fn color(&self) -> Hsla {
        self.color
    }

    /// Set show alpha
    pub fn set_show_alpha(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_alpha = show;
        cx.notify();
    }

    /// Set presets
    pub fn set_presets(&mut self, presets: Vec<Hsla>, cx: &mut Context<Self>) {
        self.presets = presets;
        cx.notify();
    }

    /// Set label
    pub fn set_label(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.label = label;
        cx.notify();
    }

    /// Set disabled
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        if disabled {
            self.is_open = false;
        }
        cx.notify();
    }

    /// Toggle picker
    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        if !self.disabled {
            self.is_open = !self.is_open;
            if self.is_open {
                cx.emit(ColorPickerEvent::Opened);
            } else {
                cx.emit(ColorPickerEvent::Closed);
            }
            cx.notify();
        }
    }

    /// Close picker
    pub fn close(&mut self, cx: &mut Context<Self>) {
        if self.is_open {
            self.is_open = false;
            cx.emit(ColorPickerEvent::Closed);
            cx.notify();
        }
    }

    /// Select preset color
    fn select_preset(&mut self, color: Hsla, cx: &mut Context<Self>) {
        self.set_color(color, cx);
    }

    /// Format color as hex
    fn color_to_hex(&self) -> String {
        let r = (self.color.l * 255.0) as u8;
        let g = (self.color.s * 255.0) as u8;
        let b = (self.color.h * 255.0 * 360.0 / 255.0) as u8;
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

impl EventEmitter<ColorPickerEvent> for ColorPicker {}

impl Focusable for ColorPicker {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ColorPicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let opacity = if self.disabled { 0.5 } else { 1.0 };

        div()
            .id("color-picker")
            .flex()
            .flex_col()
            .gap_1()
            .opacity(opacity)
            .track_focus(&self.focus_handle)
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
            // Trigger button
            .child(
                div()
                    .id("color-picker-trigger")
                    .h(px(36.0))
                    .w(px(120.0))
                    .px_2()
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(if self.is_open {
                        theme.colors.accent
                    } else {
                        theme.colors.border
                    })
                    .bg(theme.colors.surface)
                    .flex()
                    .items_center()
                    .gap_2()
                    .when(!self.disabled, |d| {
                        d.cursor_pointer()
                            .hover(|s| s.border_color(theme.colors.accent.opacity(0.5)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle(cx);
                            }))
                    })
                    // Color swatch
                    .child(
                        div()
                            .size(px(24.0))
                            .rounded(px(4.0))
                            .border_1()
                            .border_color(theme.colors.border)
                            .bg(self.color),
                    )
                    // Hex value
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(theme.colors.text)
                            .child(self.color_to_hex()),
                    )
                    // Dropdown indicator
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(if self.is_open { "▲" } else { "▼" }),
                    ),
            )
            // Dropdown panel
            .when(self.is_open, |d| {
                d.child(
                    div()
                        .id("color-picker-panel")
                        .mt_1()
                        .w(px(220.0))
                        .p_3()
                        .rounded(px(8.0))
                        .border_1()
                        .border_color(theme.colors.border)
                        .bg(theme.colors.surface)
                        .shadow_lg()
                        .flex()
                        .flex_col()
                        .gap_3()
                        // Preset colors
                        .child(div().flex().flex_wrap().gap_2().children(
                            self.presets.clone().into_iter().map(|preset| {
                                let is_selected = (preset.h - self.color.h).abs() < 0.01
                                    && (preset.s - self.color.s).abs() < 0.01;

                                div()
                                    .id(SharedString::from(format!(
                                        "preset-{:.2}-{:.2}",
                                        preset.h, preset.s
                                    )))
                                    .size(px(24.0))
                                    .rounded(px(4.0))
                                    .bg(preset)
                                    .border_2()
                                    .border_color(if is_selected {
                                        theme.colors.accent
                                    } else {
                                        preset.opacity(0.0)
                                    })
                                    .cursor_pointer()
                                    .hover(|s| s.border_color(theme.colors.accent.opacity(0.5)))
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.select_preset(preset, cx);
                                    }))
                            }),
                        ))
                        // Alpha slider (if enabled)
                        .when(self.show_alpha, |d| {
                            d.child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child("Alpha:"),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .h(px(8.0))
                                            .rounded_full()
                                            .bg(theme.colors.surface_hover),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text)
                                            .child(format!("{}%", (self.color.a * 100.0) as u8)),
                                    ),
                            )
                        }),
                )
            })
    }
}
