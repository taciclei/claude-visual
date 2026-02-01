//! Type definitions for authentication

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// OAuth provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OAuthProvider {
    /// GitHub OAuth
    GitHub,
    /// Google OAuth
    Google,
    /// Email/Password (non-OAuth)
    Email,
}

impl OAuthProvider {
    /// Get the display name
    pub fn display_name(&self) -> &'static str {
        match self {
            OAuthProvider::GitHub => "GitHub",
            OAuthProvider::Google => "Google",
            OAuthProvider::Email => "Email",
        }
    }

    /// Get the authorization URL base
    pub fn auth_url(&self) -> &'static str {
        match self {
            OAuthProvider::GitHub => "https://github.com/login/oauth/authorize",
            OAuthProvider::Google => "https://accounts.google.com/o/oauth2/v2/auth",
            OAuthProvider::Email => "",
        }
    }

    /// Get the token URL
    pub fn token_url(&self) -> &'static str {
        match self {
            OAuthProvider::GitHub => "https://github.com/login/oauth/access_token",
            OAuthProvider::Google => "https://oauth2.googleapis.com/token",
            OAuthProvider::Email => "",
        }
    }

    /// Get default scopes
    pub fn default_scopes(&self) -> Vec<&'static str> {
        match self {
            OAuthProvider::GitHub => vec!["user:email", "read:user"],
            OAuthProvider::Google => vec!["email", "profile"],
            OAuthProvider::Email => vec![],
        }
    }
}

/// User profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// Unique user ID
    pub id: String,
    /// User email
    pub email: String,
    /// Display name
    pub name: Option<String>,
    /// Avatar URL
    pub avatar_url: Option<String>,
    /// OAuth provider used
    pub provider: OAuthProvider,
    /// When the profile was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last login time
    pub last_login: chrono::DateTime<chrono::Utc>,
}

/// OAuth tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokens {
    /// Access token
    pub access_token: String,
    /// Refresh token (if available)
    pub refresh_token: Option<String>,
    /// Token expiry time
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Token type (usually "Bearer")
    pub token_type: String,
    /// Scopes granted
    pub scopes: Vec<String>,
}

impl OAuthTokens {
    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() >= expires_at
        } else {
            false
        }
    }

    /// Check if token will expire within given duration
    pub fn expires_within(&self, duration: chrono::Duration) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() + duration >= expires_at
        } else {
            false
        }
    }
}

/// Authentication state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthState {
    /// Not authenticated
    SignedOut,
    /// Authentication in progress
    Authenticating,
    /// Authenticated
    SignedIn,
    /// Authentication failed
    Failed(String),
}

/// Authentication error types
#[derive(Debug, Error)]
pub enum AuthError {
    /// OAuth flow error
    #[error("OAuth error: {0}")]
    OAuth(String),
    /// Network error
    #[error("Network error: {0}")]
    Network(String),
    /// Token refresh failed
    #[error("Token refresh failed: {0}")]
    TokenRefresh(String),
    /// Invalid credentials
    #[error("Invalid credentials")]
    InvalidCredentials,
    /// User cancelled authentication
    #[error("Authentication cancelled")]
    Cancelled,
    /// Token storage error
    #[error("Token storage error: {0}")]
    Storage(String),
    /// Unknown error
    #[error("Authentication error: {0}")]
    Unknown(String),
}

/// OAuth configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    /// Client ID
    pub client_id: String,
    /// Client secret (should be stored securely)
    pub client_secret: Option<String>,
    /// Redirect URI
    pub redirect_uri: String,
    /// Additional scopes
    pub scopes: Vec<String>,
}
