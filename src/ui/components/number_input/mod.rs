//! Number input and stepper components

mod component;
mod quantity_selector;
mod render;
mod simple_stepper;
mod spin_button;
mod types;

pub use component::NumberInput;
pub use quantity_selector::QuantitySelector;
pub use simple_stepper::SimpleStepper;
pub use spin_button::SpinButton;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_stepper() {
        let stepper = SimpleStepper::new(5, 0, 10);
        assert_eq!(stepper.value, 5);
        assert_eq!(stepper.min, 0);
        assert_eq!(stepper.max, 10);
    }

    #[test]
    fn test_quantity_selector() {
        let selector = QuantitySelector::new(3).presets(vec![1, 5, 10, 25]);

        assert_eq!(selector.value, 3);
        assert_eq!(selector.presets, vec![1, 5, 10, 25]);
    }

    #[test]
    fn test_spin_button() {
        let spin = SpinButton::new(42).label("Count");

        assert_eq!(spin.value, 42);
        assert_eq!(spin.label, Some("Count".to_string()));
    }
}
