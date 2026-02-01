//! Switch/Toggle component

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// Switch/Toggle component
pub struct Switch {
    app_state: Arc<AppState>,
    /// Whether switch is on
    checked: bool,
    /// Size variant
    size: SwitchSize,
    /// Whether switch is disabled
    disabled: bool,
    /// Label (shown to the right)
    label: Option<String>,
    /// Description (shown below label)
    description: Option<String>,
}

impl Switch {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            checked: false,
            size: SwitchSize::default(),
            disabled: false,
            label: None,
            description: None,
        }
    }

    /// Create a switch with label
    pub fn with_label(
        app_state: Arc<AppState>,
        label: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut switch = Self::new(app_state, cx);
        switch.label = Some(label.into());
        switch
    }

    /// Set checked state
    pub fn set_checked(&mut self, checked: bool, cx: &mut Context<Self>) {
        self.checked = checked;
        cx.notify();
    }

    /// Toggle checked state
    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        if !self.disabled {
            self.checked = !self.checked;
            cx.emit(SwitchEvent::Changed(self.checked));
            cx.notify();
        }
    }

    /// Set size
    pub fn set_size(&mut self, size: SwitchSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set disabled
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }

    /// Set label
    pub fn set_label(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.label = label;
        cx.notify();
    }

    /// Set description
    pub fn set_description(&mut self, description: Option<String>, cx: &mut Context<Self>) {
        self.description = description;
        cx.notify();
    }

    /// Get checked state
    pub fn is_checked(&self) -> bool {
        self.checked
    }
}

impl EventEmitter<SwitchEvent> for Switch {}

impl Render for Switch {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let track_width = self.size.track_width();
        let track_height = self.size.track_height();
        let thumb_size = self.size.thumb_size();
        let thumb_offset = if self.checked {
            self.size.thumb_offset()
        } else {
            2.0
        };

        let track_color = if self.checked {
            theme.colors.accent
        } else {
            theme.colors.surface_hover
        };

        let opacity = if self.disabled { 0.5 } else { 1.0 };

        div()
            .id("switch")
            .flex()
            .items_start()
            .gap_3()
            .opacity(opacity)
            .when(!self.disabled, |d| d.cursor_pointer())
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle(cx);
            }))
            // Switch track
            .child(
                div()
                    .w(px(track_width))
                    .h(px(track_height))
                    .rounded_full()
                    .bg(track_color)
                    .border_1()
                    .border_color(if self.checked {
                        theme.colors.accent
                    } else {
                        theme.colors.border
                    })
                    .relative()
                    .flex_shrink_0()
                    // Thumb
                    .child(
                        div()
                            .absolute()
                            .top(px(1.0))
                            .left(px(thumb_offset))
                            .size(px(thumb_size))
                            .rounded_full()
                            .bg(gpui::white())
                            .shadow_sm(),
                    ),
            )
            // Label and description
            .when(self.label.is_some() || self.description.is_some(), |d| {
                d.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_0p5()
                        .when_some(self.label.clone(), |d, label| {
                            d.child(div().text_sm().text_color(theme.colors.text).child(label))
                        })
                        .when_some(self.description.clone(), |d, desc| {
                            d.child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child(desc),
                            )
                        }),
                )
            })
    }
}
