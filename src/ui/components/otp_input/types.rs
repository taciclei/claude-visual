//! Shared types for OTP input components

/// OTP input size variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OtpSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// OTP input variant styles
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OtpVariant {
    #[default]
    Boxes,
    Underline,
    Rounded,
}

/// OTP input state
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OtpState {
    #[default]
    Default,
    Focused,
    Success,
    Error,
}
