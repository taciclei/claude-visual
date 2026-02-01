use gpui::*;
use gpui::prelude::*;
use super::label::Label;
use super::helper_text::HelperText;

/// Form field wrapper with label, input, and helper text
#[derive(IntoElement)]
pub struct FormField {
    label: Option<Label>,
    helper: Option<HelperText>,
    error: Option<SharedString>,
    input: gpui::Div,
    gap: f32,
}

impl FormField {
    pub fn new() -> Self {
        Self {
            label: None,
            helper: None,
            error: None,
            input: div(),
            gap: 4.0,
        }
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }

    pub fn helper(mut self, helper: HelperText) -> Self {
        self.helper = Some(helper);
        self
    }

    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn input(mut self, input: impl IntoElement) -> Self {
        self.input = div().child(input);
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
}

impl Default for FormField {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for FormField {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut container = div().flex().flex_col().gap(px(self.gap));

        if let Some(label) = self.label {
            container = container.child(label);
        }

        container = container.child(self.input);

        if let Some(error) = self.error {
            container = container.child(HelperText::error(error));
        } else if let Some(helper) = self.helper {
            container = container.child(helper);
        }

        container
    }
}
