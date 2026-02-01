//! Main Popover component

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// Popover component
pub struct Popover {
    pub(crate) app_state: Arc<AppState>,
    /// Whether popover is open
    pub(crate) is_open: bool,
    /// Placement
    pub(crate) placement: PopoverPlacement,
    /// Trigger mode
    pub(crate) trigger: PopoverTrigger,
    /// Content width
    pub(crate) width: Option<f32>,
    /// Whether to show arrow
    pub(crate) show_arrow: bool,
    /// Offset from trigger
    pub(crate) offset: f32,
    /// Whether clicking outside closes
    pub(crate) close_on_outside_click: bool,
    /// Whether pressing Escape closes
    pub(crate) close_on_escape: bool,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}

impl Popover {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            is_open: false,
            placement: PopoverPlacement::default(),
            trigger: PopoverTrigger::default(),
            width: None,
            show_arrow: true,
            offset: 8.0,
            close_on_outside_click: true,
            close_on_escape: true,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Open the popover
    pub fn open(&mut self, cx: &mut Context<Self>) {
        if !self.is_open {
            self.is_open = true;
            cx.emit(PopoverEvent::Opened);
            cx.notify();
        }
    }

    /// Close the popover
    pub fn close(&mut self, cx: &mut Context<Self>) {
        if self.is_open {
            self.is_open = false;
            cx.emit(PopoverEvent::Closed);
            cx.notify();
        }
    }

    /// Toggle the popover
    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        if self.is_open {
            self.close(cx);
        } else {
            self.open(cx);
        }
    }

    /// Check if open
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Set placement
    pub fn set_placement(&mut self, placement: PopoverPlacement, cx: &mut Context<Self>) {
        self.placement = placement;
        cx.notify();
    }

    /// Set trigger mode
    pub fn set_trigger(&mut self, trigger: PopoverTrigger, cx: &mut Context<Self>) {
        self.trigger = trigger;
        cx.notify();
    }

    /// Set width
    pub fn set_width(&mut self, width: Option<f32>, cx: &mut Context<Self>) {
        self.width = width;
        cx.notify();
    }

    /// Set show arrow
    pub fn set_show_arrow(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_arrow = show;
        cx.notify();
    }

    /// Set offset
    pub fn set_offset(&mut self, offset: f32, cx: &mut Context<Self>) {
        self.offset = offset;
        cx.notify();
    }

    /// Handle key events
    fn handle_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.close_on_escape && event.keystroke.key.as_str() == "escape" {
            self.close(cx);
        }
    }
}

impl EventEmitter<PopoverEvent> for Popover {}

impl Focusable for Popover {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Popover {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        // This is a simplified render - actual popover positioning
        // would need more complex layout logic
        div()
            .id("popover")
            .relative()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                this.handle_key_down(event, window, cx);
            }))
            .when(self.is_open, |d| {
                d.child(
                    div()
                        .id("popover-content")
                        .absolute()
                        .when_some(self.width, |d, w| d.w(px(w)))
                        .min_w(px(120.0))
                        .rounded(px(8.0))
                        .bg(theme.colors.surface)
                        .border_1()
                        .border_color(theme.colors.border)
                        .shadow_lg()
                        .p_2()
                        // Arrow indicator
                        .when(self.show_arrow, |d| {
                            d.child(
                                div()
                                    .absolute()
                                    .size(px(8.0))
                                    .bg(theme.colors.surface)
                                    .border_l_1()
                                    .border_t_1()
                                    .border_color(theme.colors.border),
                            )
                        }),
                )
            })
    }
}
