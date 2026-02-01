use gpui::*;

use super::{Step, StepsOrientation, StepsSize, StepsVariant};

/// Steps indicator component
#[derive(IntoElement)]
pub struct Steps {
    pub(super) id: ElementId,
    pub(super) steps: Vec<Step>,
    pub(super) current: usize,
    pub(super) orientation: StepsOrientation,
    pub(super) size: StepsSize,
    pub(super) variant: StepsVariant,
    pub(super) clickable: bool,
    pub(super) show_numbers: bool,
    pub(super) completed_color: gpui::Hsla,
    pub(super) current_color: gpui::Hsla,
    pub(super) pending_color: gpui::Hsla,
    pub(super) error_color: gpui::Hsla,
    pub(super) connector_color: gpui::Hsla,
}

impl Steps {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            steps: Vec::new(),
            current: 0,
            orientation: StepsOrientation::default(),
            size: StepsSize::default(),
            variant: StepsVariant::default(),
            clickable: false,
            show_numbers: true,
            completed_color: rgb(0x22c55e).into(),
            current_color: rgb(0x3b82f6).into(),
            pending_color: rgba(0x8888884d).into(),
            error_color: rgb(0xef4444).into(),
            connector_color: rgba(0x8888884d).into(),
        }
    }

    pub fn steps(mut self, steps: Vec<Step>) -> Self {
        self.steps = steps;
        self
    }

    pub fn add_step(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }

    pub fn current(mut self, index: usize) -> Self {
        self.current = index;
        self
    }

    pub fn orientation(mut self, orientation: StepsOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn size(mut self, size: StepsSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: StepsVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    pub fn show_numbers(mut self, show: bool) -> Self {
        self.show_numbers = show;
        self
    }

    pub fn completed_color(mut self, color: gpui::Hsla) -> Self {
        self.completed_color = color;
        self
    }

    pub fn current_color(mut self, color: gpui::Hsla) -> Self {
        self.current_color = color;
        self
    }

    pub fn pending_color(mut self, color: gpui::Hsla) -> Self {
        self.pending_color = color;
        self
    }

    pub fn error_color(mut self, color: gpui::Hsla) -> Self {
        self.error_color = color;
        self
    }
}
