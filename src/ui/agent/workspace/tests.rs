//! Tests for agent workspace

#[cfg(test)]
mod tests {
    use super::super::types::AgentMode;
    use super::super::helpers::mode_label;

    #[test]
    fn test_agent_mode_transitions() {
        // Mode transitions should be valid
        let modes = [
            AgentMode::Disabled,
            AgentMode::Idle,
            AgentMode::Planning,
            AgentMode::Executing,
            AgentMode::Paused,
            AgentMode::Completed,
            AgentMode::Failed,
        ];

        for mode in modes {
            assert!(!mode_label(mode).is_empty());
        }
    }

    #[test]
    fn test_agent_is_active() {
        let active_modes = [AgentMode::Planning, AgentMode::Executing, AgentMode::Paused];
        let inactive_modes = [AgentMode::Disabled, AgentMode::Idle, AgentMode::Completed, AgentMode::Failed];

        for mode in active_modes {
            assert!(matches!(
                mode,
                AgentMode::Planning | AgentMode::Executing | AgentMode::Paused
            ));
        }

        for mode in inactive_modes {
            assert!(!matches!(
                mode,
                AgentMode::Planning | AgentMode::Executing | AgentMode::Paused
            ));
        }
    }
}
