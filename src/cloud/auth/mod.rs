//! OAuth Authentication Module
//!
//! Handles user authentication via OAuth providers (GitHub, Google, etc.)

mod core;
mod oauth;
mod storage;
mod types;
mod utils;

// Re-export public types
pub use core::CloudAuth;
pub use types::{AuthError, AuthState, OAuthConfig, OAuthProvider, OAuthTokens, UserProfile};
