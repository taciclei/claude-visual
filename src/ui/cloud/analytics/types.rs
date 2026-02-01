//! Analytics types

use crate::cloud::team::AnalyticsPeriod;

/// Events emitted by the analytics panel
pub enum AnalyticsPanelEvent {
    /// Change period
    ChangePeriod(AnalyticsPeriod),
    /// Export analytics
    Export,
    /// Panel closed
    Closed,
}

/// Analytics view modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalyticsViewMode {
    Overview,
    Users,
    Projects,
    Timeline,
}
