//! Alert/Banner component for notifications and warnings

mod types;
mod alert;
mod inline_alert;

pub use types::*;
pub use alert::Alert;
pub use inline_alert::InlineAlert;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_type_icon() {
        assert_eq!(AlertType::Info.icon(), "ℹ️");
        assert_eq!(AlertType::Success.icon(), "✓");
        assert_eq!(AlertType::Warning.icon(), "⚠️");
        assert_eq!(AlertType::Error.icon(), "✕");
    }

    #[test]
    fn test_inline_alert_builder() {
        let alert = InlineAlert::warning("Test warning")
            .with_icon("!");

        assert_eq!(alert.message, "Test warning");
        assert_eq!(alert.alert_type, AlertType::Warning);
        assert_eq!(alert.icon, Some("!".to_string()));
    }
}
