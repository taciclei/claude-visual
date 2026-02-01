//! Variables view events

/// Events from variables view
#[derive(Debug, Clone)]
pub enum VariablesViewEvent {
    /// Expand/collapse scope
    ToggleScope(String),
    /// Expand variable
    Expand(i64),
    /// Collapse variable
    Collapse(i64),
    /// Copy value
    CopyValue(String),
    /// Set value (edit)
    SetValue { name: String, new_value: String },
    /// Watch variable
    AddWatch(String),
}
