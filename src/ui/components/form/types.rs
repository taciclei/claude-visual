//! Form type definitions

use gpui::*;
use gpui::prelude::*;

/// Form layout variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FormLayout {
    #[default]
    Vertical,
    Horizontal,
    Inline,
}

/// Form size variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FormSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Validation state
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ValidationState {
    #[default]
    None,
    Valid,
    Invalid,
    Warning,
}

/// Form field state
#[derive(Debug, Clone)]
pub struct FormFieldState {
    pub name: SharedString,
    pub value: SharedString,
    pub validation: ValidationState,
    pub error: Option<SharedString>,
    pub touched: bool,
    pub dirty: bool,
}

impl FormFieldState {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            value: "".into(),
            validation: ValidationState::None,
            error: None,
            touched: false,
            dirty: false,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn valid(mut self) -> Self {
        self.validation = ValidationState::Valid;
        self.error = None;
        self
    }

    pub fn invalid(mut self, error: impl Into<SharedString>) -> Self {
        self.validation = ValidationState::Invalid;
        self.error = Some(error.into());
        self
    }

    pub fn warning(mut self, message: impl Into<SharedString>) -> Self {
        self.validation = ValidationState::Warning;
        self.error = Some(message.into());
        self
    }

    pub fn touched(mut self, touched: bool) -> Self {
        self.touched = touched;
        self
    }

    pub fn dirty(mut self, dirty: bool) -> Self {
        self.dirty = dirty;
        self
    }
}

/// Form actions alignment
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FormActionsAlignment {
    Left,
    Center,
    #[default]
    Right,
    SpaceBetween,
}
