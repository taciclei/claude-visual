//! Password strength components
//!
//! Provides password strength indicators and validation feedback.

mod input;
mod matcher;
mod meter;
mod requirements;
mod types;

pub use input::PasswordInput;
pub use matcher::PasswordMatcher;
pub use meter::PasswordStrengthMeter;
pub use requirements::PasswordRequirements;
pub use types::{PasswordRequirement, PasswordStrength, StrengthMeterVariant};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_strength_levels() {
        assert!(PasswordStrength::VeryWeak < PasswordStrength::Weak);
        assert!(PasswordStrength::Weak < PasswordStrength::Fair);
        assert!(PasswordStrength::Fair < PasswordStrength::Good);
        assert!(PasswordStrength::Good < PasswordStrength::Strong);
    }

    #[test]
    fn test_password_strength_from_password() {
        assert_eq!(PasswordStrength::from_password(""), PasswordStrength::None);
        assert_eq!(
            PasswordStrength::from_password("abc"),
            PasswordStrength::VeryWeak
        );
        assert_eq!(
            PasswordStrength::from_password("Abc12345"),
            PasswordStrength::Fair
        );
        assert_eq!(
            PasswordStrength::from_password("MyStr0ng!Pass#2024"),
            PasswordStrength::Strong
        );
    }

    #[test]
    fn test_strength_meter_variants() {
        let bar = PasswordStrengthMeter::new("b").variant(StrengthMeterVariant::Bar);
        let segments = PasswordStrengthMeter::new("s").variant(StrengthMeterVariant::Segments);

        assert_eq!(bar.variant, StrengthMeterVariant::Bar);
        assert_eq!(segments.variant, StrengthMeterVariant::Segments);
    }

    #[test]
    fn test_password_requirement() {
        let met = PasswordRequirement::new("Has uppercase", true);
        let unmet = PasswordRequirement::new("Has special char", false);

        assert!(met.met);
        assert!(!unmet.met);
    }

    #[test]
    fn test_password_requirements_from_password() {
        let reqs = PasswordRequirements::new("r").from_password("MyPass123!");

        assert_eq!(reqs.requirements.len(), 5);
        // All requirements should be met
        assert!(reqs.requirements.iter().all(|r| r.met));
    }

    #[test]
    fn test_password_input() {
        let input = PasswordInput::new("pi")
            .value("test123")
            .show_strength(true)
            .visible(false);

        assert!(!input.visible);
        assert!(input.show_strength);
    }

    #[test]
    fn test_password_matcher() {
        let matcher = PasswordMatcher::new("pm")
            .password("secret")
            .confirmation("secret");

        assert_eq!(matcher.password, matcher.confirmation);
    }
}
