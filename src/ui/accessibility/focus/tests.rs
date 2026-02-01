//! Tests for focus management

#[cfg(test)]
mod tests {
    use crate::ui::accessibility::focus::{FocusManager, FocusTrap, FocusZone, FocusableElement};

    #[test]
    fn test_focus_navigation() {
        let mut manager = FocusManager::new();

        manager.register(FocusableElement::new("btn1", FocusZone::Main).with_tab_index(0));
        manager.register(FocusableElement::new("btn2", FocusZone::Main).with_tab_index(1));
        manager.register(FocusableElement::new("btn3", FocusZone::Main).with_tab_index(2));

        // Focus first element
        manager.focus_zone(&FocusZone::Main);
        assert_eq!(manager.current_focus.as_deref(), Some("btn1"));

        // Move to next
        manager.focus_next();
        assert_eq!(manager.current_focus.as_deref(), Some("btn2"));

        // Move to next
        manager.focus_next();
        assert_eq!(manager.current_focus.as_deref(), Some("btn3"));

        // Wrap around
        manager.focus_next();
        assert_eq!(manager.current_focus.as_deref(), Some("btn1"));

        // Move backward
        manager.focus_previous();
        assert_eq!(manager.current_focus.as_deref(), Some("btn3"));
    }

    #[test]
    fn test_focus_trap() {
        let mut manager = FocusManager::new();

        manager.register(FocusableElement::new("main1", FocusZone::Main).with_tab_index(0));
        manager.register(FocusableElement::new("modal1", FocusZone::Modal).with_tab_index(0));
        manager.register(FocusableElement::new("modal2", FocusZone::Modal).with_tab_index(1));

        // Focus main element
        manager.set_focus("main1");
        assert_eq!(manager.current_focus.as_deref(), Some("main1"));

        // Activate trap
        manager.activate_trap(FocusTrap::new(FocusZone::Modal));
        assert!(manager.is_trapped());
        assert_eq!(manager.current_focus.as_deref(), Some("modal1"));

        // Navigation stays in trap
        manager.focus_next();
        assert_eq!(manager.current_focus.as_deref(), Some("modal2"));

        manager.focus_next();
        assert_eq!(manager.current_focus.as_deref(), Some("modal1")); // Wrapped

        // Deactivate trap
        manager.deactivate_trap();
        assert!(!manager.is_trapped());
        assert_eq!(manager.current_focus.as_deref(), Some("main1")); // Restored
    }

    #[test]
    fn test_disabled_elements() {
        let mut manager = FocusManager::new();

        manager.register(FocusableElement::new("btn1", FocusZone::Main).with_tab_index(0));
        manager.register(FocusableElement::new("btn2", FocusZone::Main).with_tab_index(1));

        manager.set_enabled("btn1", false);

        manager.focus_zone(&FocusZone::Main);
        // Should skip disabled btn1 and focus btn2
        assert_eq!(manager.current_focus.as_deref(), Some("btn2"));
    }
}
