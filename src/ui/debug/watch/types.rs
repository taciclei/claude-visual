//! Watch expression data types

/// Watch expression item
#[derive(Debug, Clone)]
pub struct WatchExpression {
    /// Unique ID
    pub(crate) id: usize,
    /// Expression text
    pub(crate) expression: String,
    /// Evaluated value
    pub(crate) value: Option<String>,
    /// Value type (if known)
    pub(crate) value_type: Option<String>,
    /// Error message (if evaluation failed)
    pub(crate) error: Option<String>,
    /// Is being evaluated
    pub(crate) is_evaluating: bool,
    /// Is expanded (for complex values)
    pub(crate) expanded: bool,
    /// Children (for expandable values)
    pub(crate) children: Vec<WatchChild>,
}

impl WatchExpression {
    /// Create a new watch expression
    pub fn new(id: usize, expression: String) -> Self {
        Self {
            id,
            expression,
            value: None,
            value_type: None,
            error: None,
            is_evaluating: false,
            expanded: false,
            children: Vec::new(),
        }
    }

    /// Check if evaluation was successful
    pub fn is_success(&self) -> bool {
        self.value.is_some() && self.error.is_none()
    }

    /// Check if has error
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    /// Check if has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Get display value
    pub fn display_value(&self) -> String {
        if let Some(error) = &self.error {
            format!("<error: {}>", error)
        } else if let Some(value) = &self.value {
            if value.len() > 80 {
                format!("{}...", &value[..80])
            } else {
                value.clone()
            }
        } else if self.is_evaluating {
            "...".to_string()
        } else {
            "<not evaluated>".to_string()
        }
    }
}

/// Child value for expanded watch expressions
#[derive(Debug, Clone)]
pub struct WatchChild {
    /// Name/key
    pub name: String,
    /// Value
    pub value: String,
    /// Type
    pub value_type: Option<String>,
    /// Variables reference for further expansion
    pub variables_reference: i64,
}
