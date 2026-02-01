//! OTP Input components - One-time password and PIN inputs
//!
//! Provides components for entering verification codes and PINs.

mod cvv_input;
mod otp_input;
mod pin_input;
mod types;
mod verification_input;

pub use cvv_input::CvvInput;
pub use otp_input::OtpInput;
pub use pin_input::PinInput;
pub use types::*;
pub use verification_input::VerificationInput;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_otp_input_length() {
        let otp = OtpInput::new("otp", 6);
        assert_eq!(otp.length, 6);
    }

    #[test]
    fn test_otp_sizes() {
        let small = OtpInput::new("s", 4).size(OtpSize::Small);
        let medium = OtpInput::new("m", 4).size(OtpSize::Medium);
        let large = OtpInput::new("l", 4).size(OtpSize::Large);

        assert_eq!(small.size, OtpSize::Small);
        assert_eq!(medium.size, OtpSize::Medium);
        assert_eq!(large.size, OtpSize::Large);
    }

    #[test]
    fn test_otp_variants() {
        let boxes = OtpInput::new("b", 4).variant(OtpVariant::Boxes);
        let underline = OtpInput::new("u", 4).variant(OtpVariant::Underline);
        let rounded = OtpInput::new("r", 4).variant(OtpVariant::Rounded);

        assert_eq!(boxes.variant, OtpVariant::Boxes);
        assert_eq!(underline.variant, OtpVariant::Underline);
        assert_eq!(rounded.variant, OtpVariant::Rounded);
    }

    #[test]
    fn test_otp_states() {
        let default = OtpInput::new("d", 4).state(OtpState::Default);
        let focused = OtpInput::new("f", 4).state(OtpState::Focused);
        let success = OtpInput::new("s", 4).state(OtpState::Success);
        let error = OtpInput::new("e", 4).state(OtpState::Error);

        assert_eq!(default.state, OtpState::Default);
        assert_eq!(focused.state, OtpState::Focused);
        assert_eq!(success.state, OtpState::Success);
        assert_eq!(error.state, OtpState::Error);
    }

    #[test]
    fn test_pin_input() {
        let pin = PinInput::new("pin", 4).value("12").masked(true);

        assert_eq!(pin.length, 4);
        assert!(pin.masked);
    }

    #[test]
    fn test_verification_input() {
        let verification = VerificationInput::new("verify", 6).resend_countdown(30);

        assert_eq!(verification.length, 6);
        assert_eq!(verification.resend_countdown, Some(30));
        assert!(!verification.resend_available);
    }
}
