//! Diagnostic badge rendering

use crate::ui::sidebar::explorer_diagnostics::{DiagnosticCounts, BadgeStyle};

/// Diagnostic badge for file explorer
#[derive(Debug, Clone)]
pub struct DiagnosticBadge {
    /// Error count
    pub errors: usize,
    /// Warning count
    pub warnings: usize,
    /// Display style
    pub style: BadgeStyle,
}

impl DiagnosticBadge {
    /// Create from counts
    pub fn from_counts(counts: &DiagnosticCounts, style: BadgeStyle) -> Option<Self> {
        if !counts.has_problems() {
            return None;
        }

        Some(Self {
            errors: counts.errors,
            warnings: counts.warnings,
            style,
        })
    }

    /// Get badge text
    pub fn text(&self) -> String {
        match self.style {
            BadgeStyle::Dot => String::new(),
            BadgeStyle::Count | BadgeStyle::IconCount => {
                if self.errors > 0 && self.warnings > 0 {
                    format!("{}/{}", self.errors, self.warnings)
                } else if self.errors > 0 {
                    self.errors.to_string()
                } else {
                    self.warnings.to_string()
                }
            }
        }
    }

    /// Check if has errors
    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }

    /// Check if has warnings
    pub fn has_warnings(&self) -> bool {
        self.warnings > 0
    }
}
