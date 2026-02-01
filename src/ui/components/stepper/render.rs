//! Stepper render implementation

use gpui::*;
use gpui::prelude::*;
use super::component::Stepper;
use super::types::StepperOrientation;

impl Render for Stepper {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let is_horizontal = matches!(self.orientation, StepperOrientation::Horizontal);

        let completed_color = hsla(0.38, 0.7, 0.45, 1.0);
        let active_color = theme.colors.accent;
        let pending_color = theme.colors.text_muted;
        let error_color = theme.colors.error;
        let border_color_default = theme.colors.border;
        let surface_color = theme.colors.surface;
        let text_muted = theme.colors.text_muted;

        let current = self.current;
        let show_numbers = self.show_numbers;

        div()
            .id("stepper")
            .w_full()
            .flex()
            .when(is_horizontal, |d| d.flex_row().items_center())
            .when(!is_horizontal, |d| d.flex_col())
            .children(self.steps.iter().enumerate().map(|(index, step)| {
                let is_last = index == self.steps.len() - 1;
                let is_clickable = self.clickable && (index <= current || self.allow_back);

                if is_horizontal {
                    super::horizontal::render_horizontal_step(
                        index,
                        step,
                        is_last,
                        is_clickable,
                        current,
                        show_numbers,
                        completed_color,
                        active_color,
                        pending_color,
                        error_color,
                        border_color_default,
                        surface_color,
                        text_muted,
                        cx,
                    )
                    .into_any_element()
                } else {
                    super::vertical::render_vertical_step(
                        index,
                        step,
                        is_last,
                        is_clickable,
                        current,
                        show_numbers,
                        completed_color,
                        active_color,
                        pending_color,
                        error_color,
                        border_color_default,
                        surface_color,
                        text_muted,
                        cx,
                    )
                    .into_any_element()
                }
            }))
    }
}
