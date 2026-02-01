//! Authentication dialog types and state management

use std::sync::Arc;

use gpui::*;

use crate::app::state::AppState;
use crate::cloud::AuthState;

/// Authentication mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMode {
    SignIn,
    SignUp,
}

/// Authentication dialog
pub struct AuthDialog {
    pub(super) app_state: Arc<AppState>,
    /// Current auth state
    pub(super) auth_state: AuthState,
    /// Error message
    pub(super) error_message: Option<String>,
    /// Email input for email/password auth
    pub(super) email_input: String,
    /// Password input
    pub(super) password_input: String,
    /// Show password
    pub(super) show_password: bool,
    /// Selected auth mode
    pub(super) auth_mode: AuthMode,
    /// Focus handle
    pub(super) focus_handle: FocusHandle,
}

impl AuthDialog {
    /// Create a new auth dialog
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            auth_state: AuthState::SignedOut,
            error_message: None,
            email_input: String::new(),
            password_input: String::new(),
            show_password: false,
            auth_mode: AuthMode::SignIn,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Set auth state
    pub fn set_auth_state(&mut self, state: AuthState, cx: &mut Context<Self>) {
        self.auth_state = state;
        if let AuthState::Failed(msg) = &self.auth_state {
            self.error_message = Some(msg.clone());
        } else {
            self.error_message = None;
        }
        cx.notify();
    }

    /// Set error message
    pub fn set_error(&mut self, message: Option<String>, cx: &mut Context<Self>) {
        self.error_message = message;
        cx.notify();
    }

    /// Toggle auth mode
    pub(super) fn toggle_mode(&mut self, cx: &mut Context<Self>) {
        self.auth_mode = match self.auth_mode {
            AuthMode::SignIn => AuthMode::SignUp,
            AuthMode::SignUp => AuthMode::SignIn,
        };
        self.error_message = None;
        cx.notify();
    }

    /// Handle email input change
    pub fn set_email(&mut self, email: String, cx: &mut Context<Self>) {
        self.email_input = email;
        cx.notify();
    }

    /// Handle password input change
    pub fn set_password(&mut self, password: String, cx: &mut Context<Self>) {
        self.password_input = password;
        cx.notify();
    }

    /// Toggle password visibility
    pub fn toggle_password_visibility(&mut self, cx: &mut Context<Self>) {
        self.show_password = !self.show_password;
        cx.notify();
    }
}
