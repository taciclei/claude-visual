//! Shared types for stepper components

/// Stepper orientation
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum StepperOrientation {
    /// Horizontal (default)
    #[default]
    Horizontal,
    /// Vertical
    Vertical,
}

/// Step status
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum StepStatus {
    /// Not yet reached
    #[default]
    Pending,
    /// Currently active
    Active,
    /// Completed successfully
    Completed,
    /// Failed/error
    Error,
    /// Skipped
    Skipped,
}

/// Step data
#[derive(Debug, Clone)]
pub struct Step {
    /// Step label/title
    pub label: String,
    /// Optional description
    pub description: Option<String>,
    /// Optional icon
    pub icon: Option<String>,
    /// Status
    pub status: StepStatus,
    /// Whether step is optional
    pub optional: bool,
}

impl Step {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
            icon: None,
            status: StepStatus::default(),
            optional: false,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn status(mut self, status: StepStatus) -> Self {
        self.status = status;
        self
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn completed(mut self) -> Self {
        self.status = StepStatus::Completed;
        self
    }

    pub fn active(mut self) -> Self {
        self.status = StepStatus::Active;
        self
    }

    pub fn error(mut self) -> Self {
        self.status = StepStatus::Error;
        self
    }
}

/// Events emitted by Stepper
#[derive(Debug, Clone)]
pub enum StepperEvent {
    /// Step clicked
    StepClicked(usize),
    /// Step changed
    StepChanged { from: usize, to: usize },
    /// Completed all steps
    Completed,
}
