//! Checkbox group component

use super::checkbox::Checkbox;
use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// A group of checkboxes
#[derive(IntoElement)]
pub struct CheckboxGroup {
    name: SharedString,
    options: Vec<CheckboxOption>,
    selected: Vec<SharedString>,
    orientation: CheckboxGroupOrientation,
    size: CheckboxSize,
    gap: f32,
    disabled: bool,
    error: Option<SharedString>,
    label: Option<SharedString>,
    required: bool,
    select_all: bool,
    color: Option<Hsla>,
}

impl CheckboxGroup {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            options: Vec::new(),
            selected: Vec::new(),
            orientation: CheckboxGroupOrientation::Vertical,
            size: CheckboxSize::Medium,
            gap: 12.0,
            disabled: false,
            error: None,
            label: None,
            required: false,
            select_all: false,
            color: None,
        }
    }

    pub fn option(mut self, option: CheckboxOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn options(mut self, options: impl IntoIterator<Item = CheckboxOption>) -> Self {
        self.options.extend(options);
        self
    }

    pub fn selected(mut self, selected: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.selected = selected.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn orientation(mut self, orientation: CheckboxGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn size(mut self, size: CheckboxSize) -> Self {
        self.size = size;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn select_all(mut self, show: bool) -> Self {
        self.select_all = show;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl RenderOnce for CheckboxGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let has_error = self.error.is_some();
        let all_selected = self.selected.len() == self.options.len();
        let some_selected = !self.selected.is_empty() && !all_selected;

        let mut container = div().flex().flex_col().gap_2();

        // Label
        if let Some(label) = &self.label {
            let mut label_el = div()
                .flex()
                .items_center()
                .gap_1()
                .text_size(px(14.0))
                .text_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.9,
                    a: 1.0,
                })
                .font_weight(gpui::FontWeight::MEDIUM)
                .child(label.clone());

            if self.required {
                label_el = label_el.child(
                    div()
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.7,
                            l: 0.5,
                            a: 1.0,
                        })
                        .child("*"),
                );
            }

            container = container.child(label_el);
        }

        // Select all option
        if self.select_all {
            let state = if all_selected {
                CheckboxState::Checked
            } else if some_selected {
                CheckboxState::Indeterminate
            } else {
                CheckboxState::Unchecked
            };

            let mut select_all_cb = Checkbox::new("select-all", "Select All")
                .state(state)
                .disabled(self.disabled)
                .size(self.size);

            if let Some(color) = self.color {
                select_all_cb = select_all_cb.color(color);
            }

            container = container.child(
                div()
                    .pb_2()
                    .mb_1()
                    .border_b_1()
                    .border_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.2,
                        a: 1.0,
                    })
                    .child(select_all_cb),
            );
        }

        // Options container
        let mut options_container = div().flex().gap(px(self.gap));

        options_container = match self.orientation {
            CheckboxGroupOrientation::Vertical => options_container.flex_col(),
            CheckboxGroupOrientation::Horizontal => options_container.flex_row().flex_wrap(),
        };

        for option in &self.options {
            let is_checked = self.selected.contains(&option.id);
            let is_disabled = self.disabled || option.disabled;

            let mut checkbox = Checkbox::new(option.id.clone(), option.label.clone())
                .checked(is_checked)
                .disabled(is_disabled)
                .size(self.size)
                .error(has_error);

            if let Some(desc) = &option.description {
                checkbox = checkbox.description(desc.clone());
            }

            if let Some(color) = self.color {
                checkbox = checkbox.color(color);
            }

            options_container = options_container.child(checkbox);
        }

        container = container.child(options_container);

        // Error message
        if let Some(error) = self.error {
            container = container.child(
                div()
                    .text_size(px(12.0))
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.7,
                        l: 0.5,
                        a: 1.0,
                    })
                    .child(error),
            );
        }

        container
    }
}
