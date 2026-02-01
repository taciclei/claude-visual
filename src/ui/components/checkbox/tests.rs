//! Tests for checkbox components

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_checkbox() {
        let checkbox = Checkbox::new("accept", "Accept terms")
            .checked(true)
            .disabled(false)
            .size(CheckboxSize::Medium);
        assert_eq!(checkbox.state, CheckboxState::Checked);
        assert!(!checkbox.disabled);
    }

    #[test]
    fn test_checkbox_indeterminate() {
        let checkbox = Checkbox::new("select-all", "Select All").indeterminate();
        assert_eq!(checkbox.state, CheckboxState::Indeterminate);
    }

    #[test]
    fn test_checkbox_option() {
        let option = CheckboxOption::new("newsletter", "Subscribe to newsletter")
            .description("Receive weekly updates")
            .disabled(false);
        assert_eq!(option.id.as_ref(), "newsletter");
        assert_eq!(
            option.description.as_deref(),
            Some("Receive weekly updates")
        );
    }

    #[test]
    fn test_checkbox_group() {
        let group = CheckboxGroup::new("features")
            .option(CheckboxOption::new("dark", "Dark mode"))
            .option(CheckboxOption::new("notif", "Notifications"))
            .selected(["dark"])
            .label("Features")
            .select_all(true);
        assert_eq!(group.options.len(), 2);
        assert_eq!(group.selected.len(), 1);
        assert!(group.select_all);
    }

    #[test]
    fn test_checkbox_card_group() {
        let group = CheckboxCardGroup::new()
            .option(CheckboxCardOption::new("feat1", "Feature 1").price("$5"))
            .option(CheckboxCardOption::new("feat2", "Feature 2").price("$10"))
            .selected(["feat1"])
            .columns(2);
        assert_eq!(group.options.len(), 2);
        assert_eq!(group.columns, Some(2));
    }

    #[test]
    fn test_checkbox_toggle() {
        let toggle = CheckboxToggle::new()
            .checked(true)
            .size(CheckboxSize::Large);
        assert!(toggle.checked);
    }
}
