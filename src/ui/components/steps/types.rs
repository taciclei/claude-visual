use gpui::*;

/// Step status
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepStatus {
    #[default]
    Pending,
    Current,
    Completed,
    Error,
    Skipped,
}

/// Steps orientation
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepsOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Steps size preset
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepsSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl StepsSize {
    pub(crate) fn indicator_size(&self) -> f32 {
        match self {
            Self::Sm => 24.0,
            Self::Md => 32.0,
            Self::Lg => 40.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            Self::Sm => 12.0,
            Self::Md => 14.0,
            Self::Lg => 16.0,
        }
    }

    pub(crate) fn connector_thickness(&self) -> f32 {
        match self {
            Self::Sm => 2.0,
            Self::Md => 2.0,
            Self::Lg => 3.0,
        }
    }
}

/// Step variant style
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepsVariant {
    #[default]
    Circle,
    CircleAlt,
    Line,
    Dot,
    Icon,
}

/// Individual step definition
#[derive(Debug, Clone)]
pub struct Step {
    pub title: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<SharedString>,
    pub status: StepStatus,
}

impl Step {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            icon: None,
            status: StepStatus::Pending,
        }
    }

    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn status(mut self, status: StepStatus) -> Self {
        self.status = status;
        self
    }
}
