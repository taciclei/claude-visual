//! Helper functions for agent workspace

use gpui::*;
use super::types::{AgentMode, SimpleColors};

/// Get mode color
pub(crate) fn mode_color(mode: AgentMode, colors: &SimpleColors) -> Hsla {
    match mode {
        AgentMode::Disabled => colors.text_muted,
        AgentMode::Idle => colors.text_muted,
        AgentMode::Planning => colors.accent,
        AgentMode::Executing => colors.success,
        AgentMode::Paused => colors.warning,
        AgentMode::Completed => colors.success,
        AgentMode::Failed => colors.error,
    }
}

/// Get mode label
pub(crate) fn mode_label(mode: AgentMode) -> &'static str {
    match mode {
        AgentMode::Disabled => "Agent Off",
        AgentMode::Idle => "Agent Ready",
        AgentMode::Planning => "Planning...",
        AgentMode::Executing => "Executing",
        AgentMode::Paused => "Paused",
        AgentMode::Completed => "Completed",
        AgentMode::Failed => "Failed",
    }
}
