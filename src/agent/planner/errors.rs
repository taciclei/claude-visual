//! Plan error types

/// Plan parsing errors
#[derive(Debug, thiserror::Error)]
pub enum PlanError {
    #[error("Failed to parse plan: {0}")]
    ParseError(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("No JSON found in response")]
    NoJsonFound,
    #[error("Plan has no steps")]
    EmptyPlan,
}

/// Plan validation errors
#[derive(Debug, Clone)]
pub enum PlanValidationError {
    EmptyPlan,
    CircularDependency(usize),
    InvalidDependency(usize, usize),
    UnknownTool(String),
}
