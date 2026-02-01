//! Session storage types

use super::types::{OAuthTokens, UserProfile};
use serde::{Deserialize, Serialize};

/// Saved session data
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SavedSession {
    pub(crate) tokens: OAuthTokens,
    pub(crate) profile: UserProfile,
}
