//! Main Sheet component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Bottom sheet component
#[derive(Clone)]
pub struct Sheet {
    /// Position
    pub(crate) position: SheetPosition,
    /// Size
    size: SheetSize,
    /// Title
    pub(crate) title: Option<String>,
    /// Show drag handle
    show_drag_handle: bool,
    /// Show close button
    show_close: bool,
    /// Show backdrop
    show_backdrop: bool,
    /// Rounded corners
    rounded: bool,
}

impl Sheet {
    pub fn new() -> Self {
        Self {
            position: SheetPosition::default(),
            size: SheetSize::default(),
            title: None,
            show_drag_handle: true,
            show_close: true,
            show_backdrop: true,
            rounded: true,
        }
    }

    pub fn position(mut self, position: SheetPosition) -> Self {
        self.position = position;
        self
    }

    pub fn size(mut self, size: SheetSize) -> Self {
        self.size = size;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn show_drag_handle(mut self, show: bool) -> Self {
        self.show_drag_handle = show;
        self
    }

    pub fn show_close(mut self, show: bool) -> Self {
        self.show_close = show;
        self
    }

    pub fn show_backdrop(mut self, show: bool) -> Self {
        self.show_backdrop = show;
        self
    }

    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
        self
    }
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Sheet {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let backdrop = hsla(0.0, 0.0, 0.0, 0.5);
        let drag_handle_color = hsla(0.0, 0.0, 0.35, 1.0);

        let is_horizontal = matches!(self.position, SheetPosition::Left | SheetPosition::Right);
        let is_auto = matches!(self.size, SheetSize::Auto);

        // Backdrop
        let backdrop_el = if self.show_backdrop {
            div().absolute().inset_0().bg(backdrop).into_any_element()
        } else {
            div().into_any_element()
        };

        // Build sheet panel
        let mut sheet = div().bg(surface).flex().flex_col().overflow_hidden();

        // Size
        if is_horizontal {
            sheet = sheet.h_full();
            if !is_auto {
                sheet = sheet.w(relative(self.size.percentage()));
            } else {
                sheet = sheet.w_auto();
            }
        } else {
            sheet = sheet.w_full();
            if !is_auto {
                sheet = sheet.h(relative(self.size.percentage()));
            } else {
                sheet = sheet.h_auto().max_h(relative(0.9));
            }
        }

        // Rounded corners based on position
        if self.rounded {
            sheet = match self.position {
                SheetPosition::Bottom => sheet.rounded_t(px(16.0)),
                SheetPosition::Top => sheet.rounded_b(px(16.0)),
                SheetPosition::Left => sheet.rounded_r(px(16.0)),
                SheetPosition::Right => sheet.rounded_l(px(16.0)),
            };
        }

        // Border
        sheet = match self.position {
            SheetPosition::Bottom => sheet.border_t_1().border_color(border),
            SheetPosition::Top => sheet.border_b_1().border_color(border),
            SheetPosition::Left => sheet.border_r_1().border_color(border),
            SheetPosition::Right => sheet.border_l_1().border_color(border),
        };

        // Drag handle (for bottom sheets)
        if self.show_drag_handle && matches!(self.position, SheetPosition::Bottom) {
            sheet = sheet.child(
                div()
                    .w_full()
                    .py_3()
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(4.0))
                            .rounded(px(2.0))
                            .bg(drag_handle_color),
                    ),
            );
        }

        // Header with title and close
        let has_title = self.title.is_some();
        if has_title || self.show_close {
            sheet = sheet.child(
                div()
                    .w_full()
                    .px_4()
                    .py_3()
                    .flex()
                    .items_center()
                    .justify_between()
                    .when_some(self.title, |d, title| {
                        d.child(
                            div()
                                .text_base()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(text)
                                .child(title),
                        )
                    })
                    .when(!has_title, |d| {
                        d.child(div()) // Spacer
                    })
                    .when(self.show_close, |d| {
                        d.child(
                            div()
                                .size(px(28.0))
                                .rounded(px(4.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_sm()
                                .text_color(text_muted)
                                .cursor_pointer()
                                .hover(|s| s.bg(hsla(0.0, 0.0, 0.18, 1.0)).text_color(text))
                                .child("×"),
                        )
                    }),
            );
        }

        // Content area
        sheet = sheet.child(
            div()
                .flex_1()
                .px_4()
                .pb_4()
                .id("scroll-sheet-content")
                .overflow_y_scroll()
                .child("Sheet content..."),
        );

        // Container positioning
        let mut container = div().relative().size_full().flex();

        match self.position {
            SheetPosition::Bottom => {
                container = container.flex_col().justify_end();
            }
            SheetPosition::Top => {
                container = container.flex_col().justify_start();
            }
            SheetPosition::Left => {
                container = container.flex_row().justify_start();
            }
            SheetPosition::Right => {
                container = container.flex_row().justify_end();
            }
        }

        container.child(backdrop_el).child(sheet)
    }
}
