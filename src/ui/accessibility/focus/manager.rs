//! Focus manager for tracking and managing focus

use super::types::{FocusRingStyle, FocusTrap, FocusZone, FocusableElement};

/// Focus manager for tracking and managing focus across the application
#[derive(Debug, Default)]
pub struct FocusManager {
    /// Currently focused element ID
    pub(crate) current_focus: Option<String>,
    /// Previously focused element (for restore)
    pub(crate) previous_focus: Option<String>,
    /// Registered focusable elements
    pub(crate) elements: Vec<FocusableElement>,
    /// Active focus trap
    pub(crate) focus_trap: Option<FocusTrap>,
    /// Focus ring style
    pub(crate) ring_style: FocusRingStyle,
    /// Whether keyboard navigation mode is active
    pub(crate) keyboard_mode: bool,
}

impl FocusManager {
    /// Create a new focus manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with high contrast focus ring
    pub fn high_contrast() -> Self {
        Self {
            ring_style: FocusRingStyle::high_contrast(),
            ..Default::default()
        }
    }

    /// Register a focusable element
    pub fn register(&mut self, element: FocusableElement) {
        // Remove existing element with same ID
        self.elements.retain(|e| e.id != element.id);
        self.elements.push(element);
        self.sort_elements();
    }

    /// Unregister a focusable element
    pub fn unregister(&mut self, id: &str) {
        self.elements.retain(|e| e.id != id);
        if self.current_focus.as_deref() == Some(id) {
            self.current_focus = None;
        }
    }

    /// Update element visibility
    pub fn set_visible(&mut self, id: &str, visible: bool) {
        if let Some(elem) = self.elements.iter_mut().find(|e| e.id == id) {
            elem.visible = visible;
        }
    }

    /// Update element enabled state
    pub fn set_enabled(&mut self, id: &str, enabled: bool) {
        if let Some(elem) = self.elements.iter_mut().find(|e| e.id == id) {
            elem.enabled = enabled;
        }
    }

    /// Set the current focus
    pub fn set_focus(&mut self, id: &str) {
        if let Some(elem) = self.elements.iter().find(|e| e.id == id && e.can_focus()) {
            self.previous_focus = self.current_focus.take();
            self.current_focus = Some(elem.id.clone());
        }
    }

    /// Clear the current focus
    pub fn clear_focus(&mut self) {
        self.previous_focus = self.current_focus.take();
    }

    /// Get the currently focused element
    pub fn current_focus(&self) -> Option<&FocusableElement> {
        self.current_focus
            .as_ref()
            .and_then(|id| self.elements.iter().find(|e| &e.id == id))
    }

    /// Move focus to the next focusable element
    pub fn focus_next(&mut self) -> Option<String> {
        self.move_focus(1)
    }

    /// Move focus to the previous focusable element
    pub fn focus_previous(&mut self) -> Option<String> {
        self.move_focus(-1)
    }

    /// Focus the first element in a zone
    pub fn focus_zone(&mut self, zone: &FocusZone) -> Option<String> {
        let focusable: Vec<_> = self
            .focusable_elements()
            .filter(|e| &e.zone == zone)
            .collect();

        if let Some(elem) = focusable.first() {
            let id = elem.id.clone();
            self.set_focus(&id);
            self.current_focus.clone()
        } else {
            None
        }
    }

    /// Activate a focus trap
    pub fn activate_trap(&mut self, trap: FocusTrap) {
        self.previous_focus = self.current_focus.take();
        let zone = trap.zone.clone();
        self.focus_trap = Some(trap);
        // Focus first element in trap zone
        self.focus_zone(&zone);
    }

    /// Deactivate the focus trap
    pub fn deactivate_trap(&mut self) {
        if let Some(trap) = self.focus_trap.take() {
            if trap.restore_focus {
                if let Some(prev) = self.previous_focus.take() {
                    self.set_focus(&prev);
                }
            }
        }
    }

    /// Check if focus is trapped
    pub fn is_trapped(&self) -> bool {
        self.focus_trap.is_some()
    }

    /// Get the focus ring style
    pub fn ring_style(&self) -> &FocusRingStyle {
        &self.ring_style
    }

    /// Set the focus ring style
    pub fn set_ring_style(&mut self, style: FocusRingStyle) {
        self.ring_style = style;
    }

    /// Enable keyboard navigation mode (shows focus rings)
    pub fn enable_keyboard_mode(&mut self) {
        self.keyboard_mode = true;
    }

    /// Disable keyboard navigation mode
    pub fn disable_keyboard_mode(&mut self) {
        self.keyboard_mode = false;
    }

    /// Check if keyboard navigation mode is active
    pub fn is_keyboard_mode(&self) -> bool {
        self.keyboard_mode
    }

    // Private helpers

    fn sort_elements(&mut self) {
        self.elements.sort_by(|a, b| {
            let zone_order = a.zone.tab_order().cmp(&b.zone.tab_order());
            if zone_order == std::cmp::Ordering::Equal {
                a.tab_index.cmp(&b.tab_index)
            } else {
                zone_order
            }
        });
    }

    fn focusable_elements(&self) -> impl Iterator<Item = &FocusableElement> {
        let trap_zone = self.focus_trap.as_ref().map(|t| &t.zone);
        self.elements
            .iter()
            .filter(move |e| e.can_focus() && (trap_zone.is_none() || trap_zone == Some(&e.zone)))
    }

    fn move_focus(&mut self, direction: i32) -> Option<String> {
        let focusable: Vec<_> = self.focusable_elements().collect();
        if focusable.is_empty() {
            return None;
        }

        let current_idx = self
            .current_focus
            .as_ref()
            .and_then(|id| focusable.iter().position(|e| &e.id == id));

        let new_idx = match current_idx {
            Some(idx) => {
                let len = focusable.len() as i32;
                ((idx as i32 + direction).rem_euclid(len)) as usize
            }
            None => 0,
        };

        if let Some(elem) = focusable.get(new_idx) {
            let id = elem.id.clone();
            self.set_focus(&id);
        }

        self.current_focus.clone()
    }
}
