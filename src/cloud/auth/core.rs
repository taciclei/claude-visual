//! Core authentication manager

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::oauth::{exchange_code_for_tokens, fetch_user_profile, refresh_access_token};
use super::storage::SavedSession;
use super::types::{AuthError, AuthState, OAuthConfig, OAuthProvider, OAuthTokens, UserProfile};
use super::utils::{generate_pkce_challenge, generate_pkce_verifier, generate_state};

/// Cloud authentication manager
pub struct CloudAuth {
    /// Current auth state
    pub(crate) state: Arc<RwLock<AuthState>>,
    /// Current user profile
    pub(crate) profile: Arc<RwLock<Option<UserProfile>>>,
    /// OAuth tokens
    pub(crate) tokens: Arc<RwLock<Option<OAuthTokens>>>,
    /// OAuth configurations per provider
    pub(crate) configs: HashMap<OAuthProvider, OAuthConfig>,
    /// Token storage path
    pub(crate) token_path: PathBuf,
    /// PKCE code verifier for current flow
    pub(crate) pkce_verifier: Arc<RwLock<Option<String>>>,
}

impl CloudAuth {
    /// Create a new authentication manager
    pub fn new(config_dir: PathBuf) -> Self {
        let token_path = config_dir.join("auth_tokens.json");

        Self {
            state: Arc::new(RwLock::new(AuthState::SignedOut)),
            profile: Arc::new(RwLock::new(None)),
            tokens: Arc::new(RwLock::new(None)),
            configs: HashMap::new(),
            token_path,
            pkce_verifier: Arc::new(RwLock::new(None)),
        }
    }

    /// Configure an OAuth provider
    pub fn configure_provider(&mut self, provider: OAuthProvider, config: OAuthConfig) {
        self.configs.insert(provider, config);
    }

    /// Get current auth state
    pub async fn state(&self) -> AuthState {
        self.state.read().await.clone()
    }

    /// Get current user profile
    pub async fn profile(&self) -> Option<UserProfile> {
        self.profile.read().await.clone()
    }

    /// Check if user is authenticated
    pub async fn is_authenticated(&self) -> bool {
        matches!(*self.state.read().await, AuthState::SignedIn)
    }

    /// Start OAuth flow for a provider
    pub async fn start_oauth_flow(&self, provider: OAuthProvider) -> Result<String, AuthError> {
        let config = self
            .configs
            .get(&provider)
            .ok_or_else(|| AuthError::OAuth(format!("Provider {:?} not configured", provider)))?;

        // Generate PKCE code verifier and challenge
        let verifier = generate_pkce_verifier();
        let challenge = generate_pkce_challenge(&verifier);

        // Store verifier for later
        *self.pkce_verifier.write().await = Some(verifier);

        // Update state
        *self.state.write().await = AuthState::Authenticating;

        // Build authorization URL
        let mut scopes = provider.default_scopes();
        for scope in &config.scopes {
            if !scopes.contains(&scope.as_str()) {
                scopes.push(scope.as_str());
            }
        }

        let state = generate_state();

        let auth_url = format!(
            "{}?client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256&response_type=code",
            provider.auth_url(),
            urlencoding::encode(&config.client_id),
            urlencoding::encode(&config.redirect_uri),
            urlencoding::encode(&scopes.join(" ")),
            urlencoding::encode(&state),
            urlencoding::encode(&challenge),
        );

        Ok(auth_url)
    }

    /// Handle OAuth callback with authorization code
    pub async fn handle_oauth_callback(
        &self,
        provider: OAuthProvider,
        code: &str,
    ) -> Result<UserProfile, AuthError> {
        let config = self
            .configs
            .get(&provider)
            .ok_or_else(|| AuthError::OAuth(format!("Provider {:?} not configured", provider)))?;

        let verifier = self
            .pkce_verifier
            .read()
            .await
            .clone()
            .ok_or_else(|| AuthError::OAuth("No PKCE verifier found".to_string()))?;

        // Exchange code for tokens
        let tokens = exchange_code_for_tokens(provider, config, code, &verifier).await?;

        // Fetch user profile
        let profile = fetch_user_profile(provider, &tokens).await?;

        // Store tokens and profile
        *self.tokens.write().await = Some(tokens.clone());
        *self.profile.write().await = Some(profile.clone());
        *self.state.write().await = AuthState::SignedIn;

        // Clear PKCE verifier
        *self.pkce_verifier.write().await = None;

        // Persist tokens
        self.save_tokens(&tokens).await?;

        Ok(profile)
    }

    /// Refresh access token
    pub async fn refresh_token(&self) -> Result<(), AuthError> {
        let tokens = self
            .tokens
            .read()
            .await
            .clone()
            .ok_or(AuthError::InvalidCredentials)?;

        let refresh_token = tokens
            .refresh_token
            .ok_or_else(|| AuthError::TokenRefresh("No refresh token available".to_string()))?;

        let profile = self
            .profile
            .read()
            .await
            .clone()
            .ok_or(AuthError::InvalidCredentials)?;

        let config = self
            .configs
            .get(&profile.provider)
            .ok_or_else(|| AuthError::OAuth("Provider not configured".to_string()))?;

        let new_tokens = refresh_access_token(profile.provider, config, &refresh_token).await?;

        *self.tokens.write().await = Some(new_tokens.clone());
        self.save_tokens(&new_tokens).await?;

        Ok(())
    }

    /// Sign out
    pub async fn sign_out(&self) -> Result<(), AuthError> {
        *self.state.write().await = AuthState::SignedOut;
        *self.profile.write().await = None;
        *self.tokens.write().await = None;

        // Remove stored tokens
        if self.token_path.exists() {
            tokio::fs::remove_file(&self.token_path)
                .await
                .map_err(|e| AuthError::Storage(e.to_string()))?;
        }

        Ok(())
    }

    /// Load saved tokens from disk
    pub async fn load_saved_session(&self) -> Result<bool, AuthError> {
        if !self.token_path.exists() {
            return Ok(false);
        }

        let data = tokio::fs::read_to_string(&self.token_path)
            .await
            .map_err(|e| AuthError::Storage(e.to_string()))?;

        let saved: SavedSession =
            serde_json::from_str(&data).map_err(|e| AuthError::Storage(e.to_string()))?;

        // Check if tokens are still valid
        if saved.tokens.is_expired() {
            // Try to refresh
            if saved.tokens.refresh_token.is_some() {
                *self.tokens.write().await = Some(saved.tokens);
                *self.profile.write().await = Some(saved.profile);
                self.refresh_token().await?;
                *self.state.write().await = AuthState::SignedIn;
                return Ok(true);
            }
            return Ok(false);
        }

        *self.tokens.write().await = Some(saved.tokens);
        *self.profile.write().await = Some(saved.profile);
        *self.state.write().await = AuthState::SignedIn;

        Ok(true)
    }

    /// Get current access token
    pub async fn access_token(&self) -> Option<String> {
        let tokens = self.tokens.read().await;
        tokens.as_ref().map(|t| t.access_token.clone())
    }

    /// Save tokens to disk
    async fn save_tokens(&self, tokens: &OAuthTokens) -> Result<(), AuthError> {
        let profile = self
            .profile
            .read()
            .await
            .clone()
            .ok_or(AuthError::Storage("No profile to save".to_string()))?;

        let saved = SavedSession {
            tokens: tokens.clone(),
            profile,
        };

        let data =
            serde_json::to_string_pretty(&saved).map_err(|e| AuthError::Storage(e.to_string()))?;

        // Ensure directory exists
        if let Some(parent) = self.token_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| AuthError::Storage(e.to_string()))?;
        }

        tokio::fs::write(&self.token_path, data)
            .await
            .map_err(|e| AuthError::Storage(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::OAuthTokens;

    #[test]
    fn test_token_expiry() {
        let tokens = OAuthTokens {
            access_token: "test".to_string(),
            refresh_token: None,
            expires_at: Some(chrono::Utc::now() - chrono::Duration::hours(1)),
            token_type: "Bearer".to_string(),
            scopes: vec![],
        };

        assert!(tokens.is_expired());

        let valid_tokens = OAuthTokens {
            access_token: "test".to_string(),
            refresh_token: None,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            token_type: "Bearer".to_string(),
            scopes: vec![],
        };

        assert!(!valid_tokens.is_expired());
    }
}
