#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_radio() {
        let radio = Radio::new("opt1", "Option 1")
            .checked(true)
            .disabled(false)
            .size(RadioSize::Medium);
        assert!(radio.checked);
        assert!(!radio.disabled);
    }

    #[test]
    fn test_radio_option() {
        let option = RadioOption::new("value", "Label")
            .description("A description")
            .disabled(false);
        assert_eq!(option.value.as_ref(), "value");
        assert_eq!(option.description.as_deref(), Some("A description"));
    }

    #[test]
    fn test_radio_group() {
        let group = RadioGroup::new("payment")
            .option(RadioOption::new("card", "Credit Card"))
            .option(RadioOption::new("paypal", "PayPal"))
            .selected("card")
            .label("Payment Method")
            .required(true);
        assert_eq!(group.options.len(), 2);
        assert_eq!(group.selected.as_deref(), Some("card"));
        assert!(group.required);
    }

    #[test]
    fn test_radio_card_group() {
        let group = RadioCardGroup::new()
            .option(RadioCardOption::new("basic", "Basic").price("$9/mo"))
            .option(
                RadioCardOption::new("pro", "Pro")
                    .price("$19/mo")
                    .badge("Popular"),
            )
            .selected("pro")
            .columns(2);
        assert_eq!(group.options.len(), 2);
        assert_eq!(group.columns, Some(2));
    }

    #[test]
    fn test_inline_radio() {
        let radio = InlineRadio::new()
            .option("yes", "Yes")
            .option("no", "No")
            .selected("yes");
        assert_eq!(radio.options.len(), 2);
        assert_eq!(radio.selected.as_deref(), Some("yes"));
    }
}
