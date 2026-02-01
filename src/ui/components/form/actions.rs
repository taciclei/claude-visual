//! Form actions - container for submit/cancel buttons

use gpui::*;
use gpui::prelude::*;
use super::types::FormActionsAlignment;

#[derive(IntoElement)]
pub struct FormActions {
    children: Vec<gpui::AnyElement>,
    alignment: FormActionsAlignment,
    sticky: bool,
}

impl FormActions {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            alignment: FormActionsAlignment::default(),
            sticky: false,
        }
    }

    pub fn children(mut self, children: Vec<impl IntoElement>) -> Self {
        self.children = children.into_iter().map(|c| c.into_any_element()).collect();
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn alignment(mut self, alignment: FormActionsAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn sticky(mut self, sticky: bool) -> Self {
        self.sticky = sticky;
        self
    }
}

impl Default for FormActions {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for FormActions {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut actions = div()
            .flex()
            .items_center()
            .gap(px(12.0))
            .pt(px(16.0));

        actions = match self.alignment {
            FormActionsAlignment::Left => actions,
            FormActionsAlignment::Center => actions.justify_center(),
            FormActionsAlignment::Right => actions.justify_end(),
            FormActionsAlignment::SpaceBetween => actions.justify_between(),
        };

        if self.sticky {
            actions = actions
                .absolute()
                .bottom_0()
                .py(px(16.0))
                .bg(hsla(0.0, 0.0, 0.1, 0.95))
                .border_t_1()
                .border_color(hsla(0.0, 0.0, 0.2, 1.0));
        }

        actions.children(self.children)
    }
}
