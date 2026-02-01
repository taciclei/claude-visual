//! Variable data structures

/// Variable item for display
#[derive(Debug, Clone)]
pub struct VariableItem {
    /// Variable name
    pub name: String,
    /// Variable value (as string)
    pub value: String,
    /// Variable type
    pub var_type: Option<String>,
    /// Variables reference (for expansion)
    pub variables_reference: i64,
    /// Is expanded
    pub expanded: bool,
    /// Children (if expanded)
    pub children: Vec<VariableItem>,
    /// Depth in tree
    pub depth: usize,
}

impl VariableItem {
    /// Create a simple variable
    pub fn new(name: String, value: String, var_type: Option<String>) -> Self {
        Self {
            name,
            value,
            var_type,
            variables_reference: 0,
            expanded: false,
            children: Vec::new(),
            depth: 0,
        }
    }

    /// Check if variable has children
    pub fn has_children(&self) -> bool {
        self.variables_reference > 0
    }

    /// Get display value (truncated if long)
    pub fn display_value(&self) -> String {
        if self.value.len() > 100 {
            format!("{}...", &self.value[..100])
        } else {
            self.value.clone()
        }
    }
}

/// Scope for grouping variables
#[derive(Debug, Clone)]
pub struct ScopeItem {
    /// Scope name
    pub name: String,
    /// Variables in this scope
    pub variables: Vec<VariableItem>,
    /// Is expanded
    pub expanded: bool,
}

impl ScopeItem {
    pub fn new(name: String) -> Self {
        Self {
            name,
            variables: Vec::new(),
            expanded: true,
        }
    }
}
