//! Session storage types

use serde::{Deserialize, Serialize};
use super::types::{OAuthTokens, UserProfile};

/// Saved session data
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SavedSession {
    pub(crate) tokens: OAuthTokens,
    pub(crate) profile: UserProfile,
}
