//! Authentication events

use gpui::*;

use crate::cloud::OAuthProvider;

/// Events emitted by the auth dialog
pub enum AuthDialogEvent {
    /// User selected a provider to sign in
    SignIn(OAuthProvider),
    /// User wants to sign out
    SignOut,
    /// Dialog closed
    Closed,
    /// OAuth callback received
    OAuthCallback { code: String, state: String },
}
