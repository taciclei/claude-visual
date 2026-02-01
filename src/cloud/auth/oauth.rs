//! OAuth flow operations

use serde::Deserialize;
use std::collections::HashMap;
use super::types::{AuthError, OAuthConfig, OAuthProvider, OAuthTokens, UserProfile};

/// Exchange authorization code for tokens
pub(crate) async fn exchange_code_for_tokens(
    provider: OAuthProvider,
    config: &OAuthConfig,
    code: &str,
    verifier: &str,
) -> Result<OAuthTokens, AuthError> {
    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("client_id", config.client_id.as_str());
    params.insert("code", code);
    params.insert("redirect_uri", config.redirect_uri.as_str());
    params.insert("code_verifier", verifier);
    params.insert("grant_type", "authorization_code");

    if let Some(secret) = &config.client_secret {
        params.insert("client_secret", secret.as_str());
    }

    let response = client
        .post(provider.token_url())
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await
        .map_err(|e| AuthError::Network(e.to_string()))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AuthError::OAuth(format!("Token exchange failed: {}", error_text)));
    }

    let token_response: TokenResponse = response
        .json()
        .await
        .map_err(|e| AuthError::OAuth(format!("Invalid token response: {}", e)))?;

    let expires_at = token_response.expires_in.map(|secs| {
        chrono::Utc::now() + chrono::Duration::seconds(secs as i64)
    });

    Ok(OAuthTokens {
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token,
        expires_at,
        token_type: token_response.token_type.unwrap_or_else(|| "Bearer".to_string()),
        scopes: token_response
            .scope
            .map(|s| s.split_whitespace().map(String::from).collect())
            .unwrap_or_default(),
    })
}

/// Refresh access token
pub(crate) async fn refresh_access_token(
    provider: OAuthProvider,
    config: &OAuthConfig,
    refresh_token: &str,
) -> Result<OAuthTokens, AuthError> {
    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("client_id", config.client_id.as_str());
    params.insert("refresh_token", refresh_token);
    params.insert("grant_type", "refresh_token");

    if let Some(secret) = &config.client_secret {
        params.insert("client_secret", secret.as_str());
    }

    let response = client
        .post(provider.token_url())
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await
        .map_err(|e| AuthError::Network(e.to_string()))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AuthError::TokenRefresh(format!(
            "Token refresh failed: {}",
            error_text
        )));
    }

    let token_response: TokenResponse = response
        .json()
        .await
        .map_err(|e| AuthError::TokenRefresh(format!("Invalid token response: {}", e)))?;

    let expires_at = token_response.expires_in.map(|secs| {
        chrono::Utc::now() + chrono::Duration::seconds(secs as i64)
    });

    Ok(OAuthTokens {
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token.or_else(|| Some(refresh_token.to_string())),
        expires_at,
        token_type: token_response.token_type.unwrap_or_else(|| "Bearer".to_string()),
        scopes: token_response
            .scope
            .map(|s| s.split_whitespace().map(String::from).collect())
            .unwrap_or_default(),
    })
}

/// Fetch user profile from provider
pub(crate) async fn fetch_user_profile(
    provider: OAuthProvider,
    tokens: &OAuthTokens,
) -> Result<UserProfile, AuthError> {
    let client = reqwest::Client::new();

    let (url, email_url) = match provider {
        OAuthProvider::GitHub => (
            "https://api.github.com/user",
            Some("https://api.github.com/user/emails"),
        ),
        OAuthProvider::Google => (
            "https://www.googleapis.com/oauth2/v2/userinfo",
            None,
        ),
        OAuthProvider::Email => return Err(AuthError::OAuth("Email provider not supported for profile fetch".to_string())),
    };

    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", tokens.access_token))
        .header("User-Agent", "claude-visual")
        .send()
        .await
        .map_err(|e| AuthError::Network(e.to_string()))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AuthError::OAuth(format!("Profile fetch failed: {}", error_text)));
    }

    let profile_data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AuthError::OAuth(format!("Invalid profile response: {}", e)))?;

    let now = chrono::Utc::now();

    match provider {
        OAuthProvider::GitHub => {
            let mut email = profile_data["email"].as_str().map(String::from);

            // Fetch email separately if not in profile
            if email.is_none() {
                if let Some(email_url) = email_url {
                    if let Ok(email_response) = client
                        .get(email_url)
                        .header("Authorization", format!("Bearer {}", tokens.access_token))
                        .header("User-Agent", "claude-visual")
                        .send()
                        .await
                    {
                        if let Ok(emails) = email_response.json::<Vec<GitHubEmail>>().await {
                            email = emails
                                .into_iter()
                                .find(|e| e.primary)
                                .map(|e| e.email);
                        }
                    }
                }
            }

            Ok(UserProfile {
                id: profile_data["id"].to_string(),
                email: email.unwrap_or_default(),
                name: profile_data["name"].as_str().map(String::from),
                avatar_url: profile_data["avatar_url"].as_str().map(String::from),
                provider,
                created_at: now,
                last_login: now,
            })
        }
        OAuthProvider::Google => {
            Ok(UserProfile {
                id: profile_data["id"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                email: profile_data["email"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                name: profile_data["name"].as_str().map(String::from),
                avatar_url: profile_data["picture"].as_str().map(String::from),
                provider,
                created_at: now,
                last_login: now,
            })
        }
        OAuthProvider::Email => Err(AuthError::OAuth("Email provider not supported".to_string())),
    }
}

/// Token response from OAuth provider
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<u64>,
    token_type: Option<String>,
    scope: Option<String>,
}

/// GitHub email response
#[derive(Debug, Deserialize)]
struct GitHubEmail {
    email: String,
    primary: bool,
    #[allow(dead_code)]
    verified: bool,
}
