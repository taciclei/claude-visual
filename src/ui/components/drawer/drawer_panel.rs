//! Basic drawer panel component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Slide-out drawer panel
#[derive(Clone)]
pub struct DrawerPanel {
    /// Position/direction
    pub(crate) position: DrawerPosition,
    /// Size
    pub(crate) size: DrawerSize,
    /// Title
    pub(crate) title: Option<String>,
    /// Show close button
    pub(crate) show_close: bool,
    /// Show backdrop/overlay
    pub(crate) show_backdrop: bool,
    /// Header content (element ID)
    pub(crate) header: Option<String>,
    /// Footer content
    pub(crate) footer: Option<String>,
}

impl DrawerPanel {
    pub fn new() -> Self {
        Self {
            position: DrawerPosition::default(),
            size: DrawerSize::default(),
            title: None,
            show_close: true,
            show_backdrop: true,
            header: None,
            footer: None,
        }
    }

    pub fn position(mut self, position: DrawerPosition) -> Self {
        self.position = position;
        self
    }

    pub fn size(mut self, size: DrawerSize) -> Self {
        self.size = size;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
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

    pub fn with_footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = Some(footer.into());
        self
    }
}

impl Default for DrawerPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for DrawerPanel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let backdrop = hsla(0.0, 0.0, 0.0, 0.5);

        let is_horizontal = matches!(self.position, DrawerPosition::Left | DrawerPosition::Right);

        // Backdrop overlay
        let backdrop_el = if self.show_backdrop {
            div()
                .absolute()
                .inset_0()
                .bg(backdrop)
                .into_any_element()
        } else {
            div().into_any_element()
        };

        // Build drawer panel
        let mut drawer = div()
            .bg(surface)
            .flex()
            .flex_col()
            .overflow_hidden();

        // Apply size
        if is_horizontal {
            drawer = drawer
                .w(self.size.width(self.position))
                .h_full();
        } else {
            drawer = drawer
                .w_full()
                .h(self.size.height(self.position));
        }

        // Apply border based on position
        drawer = match self.position {
            DrawerPosition::Left => drawer.border_r_1().border_color(border),
            DrawerPosition::Right => drawer.border_l_1().border_color(border),
            DrawerPosition::Top => drawer.border_b_1().border_color(border),
            DrawerPosition::Bottom => drawer.border_t_1().border_color(border),
        };

        // Header
        let header = div()
            .w_full()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(border)
            .flex()
            .items_center()
            .justify_between()
            .when_some(self.title.clone(), |d, title| {
                d.child(
                    div()
                        .text_base()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text)
                        .child(title)
                )
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
                        .child("×")
                )
            });

        // Content area
        let content = div()
            .flex_1()
            .p_4()
            .id("scroll-drawer-content")
            .overflow_y_scroll()
            .child("Drawer content goes here...");

        // Footer (if provided)
        let footer_el = if let Some(footer_text) = self.footer {
            div()
                .w_full()
                .px_4()
                .py_3()
                .border_t_1()
                .border_color(border)
                .flex()
                .items_center()
                .justify_end()
                .gap_2()
                .child(
                    div()
                        .text_sm()
                        .text_color(text_muted)
                        .child(footer_text)
                )
                .into_any_element()
        } else {
            div().into_any_element()
        };

        drawer = drawer
            .child(header)
            .child(content)
            .child(footer_el);

        // Container with backdrop
        let mut container = div()
            .relative()
            .size_full()
            .flex();

        if is_horizontal {
            container = container.flex_row();
            if matches!(self.position, DrawerPosition::Right) {
                container = container.justify_end();
            }
        } else {
            container = container.flex_col();
            if matches!(self.position, DrawerPosition::Bottom) {
                container = container.justify_end();
            }
        }

        container
            .child(backdrop_el)
            .child(drawer)
    }
}
