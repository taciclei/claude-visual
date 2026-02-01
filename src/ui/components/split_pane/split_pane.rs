//! Main split pane component with resizable panels

use std::sync::Arc;

use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;
use super::types::*;

/// Split pane component
pub struct SplitPane {
    app_state: Arc<AppState>,
    /// Orientation
    pub(crate) orientation: SplitOrientation,
    /// Split position (0.0 to 1.0)
    pub(crate) position: f32,
    /// Minimum size of first pane (in pixels)
    pub(crate) min_first: f32,
    /// Minimum size of second pane (in pixels)
    pub(crate) min_second: f32,
    /// Default split position
    pub(crate) default_position: f32,
    /// Whether first pane is collapsed
    pub(crate) first_collapsed: bool,
    /// Whether second pane is collapsed
    pub(crate) second_collapsed: bool,
    /// Whether resizing is enabled
    pub(crate) resizable: bool,
    /// Whether the divider shows a handle
    pub(crate) show_handle: bool,
    /// Divider thickness
    pub(crate) divider_size: f32,
    /// Whether currently dragging
    pub(crate) is_dragging: bool,
}

impl SplitPane {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            orientation: SplitOrientation::default(),
            position: 0.5,
            min_first: 100.0,
            min_second: 100.0,
            default_position: 0.5,
            first_collapsed: false,
            second_collapsed: false,
            resizable: true,
            show_handle: true,
            divider_size: 4.0,
            is_dragging: false,
        }
    }

    /// Set orientation
    pub fn set_orientation(&mut self, orientation: SplitOrientation, cx: &mut Context<Self>) {
        self.orientation = orientation;
        cx.notify();
    }

    /// Set position (0.0 to 1.0)
    pub fn set_position(&mut self, position: f32, cx: &mut Context<Self>) {
        self.position = position.clamp(0.0, 1.0);
        cx.emit(SplitPaneEvent::Resized { position: self.position });
        cx.notify();
    }

    /// Get current position
    pub fn position(&self) -> f32 {
        self.position
    }

    /// Set minimum first pane size
    pub fn set_min_first(&mut self, min: f32, cx: &mut Context<Self>) {
        self.min_first = min.max(0.0);
        cx.notify();
    }

    /// Set minimum second pane size
    pub fn set_min_second(&mut self, min: f32, cx: &mut Context<Self>) {
        self.min_second = min.max(0.0);
        cx.notify();
    }

    /// Set default position
    pub fn set_default_position(&mut self, position: f32, cx: &mut Context<Self>) {
        self.default_position = position.clamp(0.0, 1.0);
        cx.notify();
    }

    /// Set resizable
    pub fn set_resizable(&mut self, resizable: bool, cx: &mut Context<Self>) {
        self.resizable = resizable;
        cx.notify();
    }

    /// Set show handle
    pub fn set_show_handle(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_handle = show;
        cx.notify();
    }

    /// Set divider size
    pub fn set_divider_size(&mut self, size: f32, cx: &mut Context<Self>) {
        self.divider_size = size.clamp(1.0, 20.0);
        cx.notify();
    }

    /// Collapse first pane
    pub fn collapse_first(&mut self, cx: &mut Context<Self>) {
        self.first_collapsed = true;
        self.second_collapsed = false;
        cx.emit(SplitPaneEvent::Collapsed);
        cx.notify();
    }

    /// Collapse second pane
    pub fn collapse_second(&mut self, cx: &mut Context<Self>) {
        self.second_collapsed = true;
        self.first_collapsed = false;
        cx.emit(SplitPaneEvent::Collapsed);
        cx.notify();
    }

    /// Expand all panes
    pub fn expand(&mut self, cx: &mut Context<Self>) {
        self.first_collapsed = false;
        self.second_collapsed = false;
        cx.emit(SplitPaneEvent::Expanded);
        cx.notify();
    }

    /// Reset to default position
    pub fn reset(&mut self, cx: &mut Context<Self>) {
        self.position = self.default_position;
        self.first_collapsed = false;
        self.second_collapsed = false;
        cx.emit(SplitPaneEvent::Resized { position: self.position });
        cx.notify();
    }

    /// Is first pane visible
    pub fn is_first_visible(&self) -> bool {
        !self.first_collapsed
    }

    /// Is second pane visible
    pub fn is_second_visible(&self) -> bool {
        !self.second_collapsed
    }
}

impl EventEmitter<SplitPaneEvent> for SplitPane {}

impl Render for SplitPane {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        let is_horizontal = matches!(self.orientation, SplitOrientation::Horizontal);
        let divider_size = self.divider_size;

        // Calculate pane sizes based on position
        let first_flex = if self.first_collapsed {
            0.0
        } else if self.second_collapsed {
            1.0
        } else {
            self.position
        };

        let second_flex = if self.second_collapsed {
            0.0
        } else if self.first_collapsed {
            1.0
        } else {
            1.0 - self.position
        };

        let cursor_style = if self.resizable {
            if is_horizontal { CursorStyle::ResizeLeftRight } else { CursorStyle::ResizeUpDown }
        } else {
            CursorStyle::Arrow
        };

        div()
            .id("split-pane")
            .w_full()
            .h_full()
            .flex()
            .when(is_horizontal, |d| d.flex_row())
            .when(!is_horizontal, |d| d.flex_col())
            // First pane
            .when(!self.first_collapsed, |d| {
                d.child(
                    div()
                        .id("split-first")
                        .when(is_horizontal, |d| d.h_full())
                        .when(!is_horizontal, |d| d.w_full())
                        .flex_basis(Length::Definite(px(0.0).into()))
                        .flex_grow()
                        .min_w(px(self.min_first))
                        .overflow_hidden()
                        .child(
                            div()
                                .w_full()
                                .h_full()
                                .text_sm()
                                .text_color(theme.colors.text_muted)
                                .flex()
                                .items_center()
                                .justify_center()
                                .child("First Pane")
                        )
                )
            })
            // Divider
            .when(!self.first_collapsed && !self.second_collapsed, |d| {
                d.child(
                    div()
                        .id("split-divider")
                        .when(is_horizontal, |d| {
                            d.w(px(divider_size))
                                .h_full()
                        })
                        .when(!is_horizontal, |d| {
                            d.h(px(divider_size))
                                .w_full()
                        })
                        .bg(theme.colors.border)
                        .flex_shrink_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor(cursor_style)
                        .when(self.resizable, |d| {
                            d.hover(|s| s.bg(theme.colors.accent.opacity(0.5)))
                        })
                        // Handle indicator
                        .when(self.show_handle, |d| {
                            d.child(
                                div()
                                    .when(is_horizontal, |d| {
                                        d.w(px(2.0)).h(px(24.0))
                                    })
                                    .when(!is_horizontal, |d| {
                                        d.h(px(2.0)).w(px(24.0))
                                    })
                                    .rounded_full()
                                    .bg(theme.colors.text_muted.opacity(0.3))
                            )
                        })
                )
            })
            // Second pane
            .when(!self.second_collapsed, |d| {
                d.child(
                    div()
                        .id("split-second")
                        .when(is_horizontal, |d| d.h_full())
                        .when(!is_horizontal, |d| d.w_full())
                        .flex_basis(Length::Definite(px(0.0).into()))
                        .flex_grow()
                        .min_w(px(self.min_second))
                        .overflow_hidden()
                        .child(
                            div()
                                .w_full()
                                .h_full()
                                .text_sm()
                                .text_color(theme.colors.text_muted)
                                .flex()
                                .items_center()
                                .justify_center()
                                .child("Second Pane")
                        )
                )
            })
    }
}
