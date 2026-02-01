//! Chip and filter components

pub mod types;
pub mod chip;
pub mod filter_chip;
pub mod filter_chip_group;
pub mod input_chips;
pub mod choice_chips;

// Re-export public items
pub use types::*;
pub use chip::Chip;
pub use filter_chip::FilterChip;
pub use filter_chip_group::FilterChipGroup;
pub use input_chips::InputChips;
pub use choice_chips::ChoiceChips;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_chip() {
        let chip = FilterChip::new("Active")
            .selected()
            .count(5);

        assert_eq!(chip.label, "Active");
        assert!(chip.selected);
        assert_eq!(chip.count, Some(5));
    }

    #[test]
    fn test_filter_chip_group() {
        let group = FilterChipGroup::new()
            .chip(FilterChip::new("All"))
            .chip(FilterChip::new("Active").selected())
            .allow_multiple();

        assert_eq!(group.chips.len(), 2);
        assert!(group.allow_multiple);
    }

    #[test]
    fn test_input_chips() {
        let chips = InputChips::new()
            .value("alice@example.com")
            .value("bob@example.com")
            .placeholder("Add email");

        assert_eq!(chips.values.len(), 2);
        assert_eq!(chips.placeholder, "Add email");
    }

    #[test]
    fn test_choice_chips() {
        let chips = ChoiceChips::new(vec!["Small", "Medium", "Large"])
            .selected(1);

        assert_eq!(chips.options.len(), 3);
        assert_eq!(chips.selected, Some(1));
    }
}
