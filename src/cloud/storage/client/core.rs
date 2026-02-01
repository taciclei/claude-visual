//! Core CloudStorage struct and initialization

use super::super::types::EncryptionAlgorithm;

/// Cloud storage client
pub struct CloudStorage {
    /// Base API URL
    pub(crate) base_url: String,
    /// Access token
    pub(crate) access_token: Option<String>,
    /// HTTP client
    pub(crate) client: reqwest::Client,
    /// Encryption key (derived from user password)
    pub(crate) encryption_key: Option<Vec<u8>>,
    /// Encryption algorithm
    pub(crate) algorithm: EncryptionAlgorithm,
}

impl CloudStorage {
    /// Create a new cloud storage client
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            access_token: None,
            client: reqwest::Client::new(),
            encryption_key: None,
            algorithm: EncryptionAlgorithm::default(),
        }
    }
}
