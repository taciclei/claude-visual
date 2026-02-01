//! Authentication and encryption key management

use super::super::crypto::derive_key;
use super::super::types::StorageError;
use super::core::CloudStorage;

impl CloudStorage {
    /// Set access token
    pub fn set_access_token(&mut self, token: String) {
        self.access_token = Some(token);
    }

    /// Set encryption key from password
    pub fn set_encryption_key(&mut self, password: &str, salt: &[u8]) -> Result<(), StorageError> {
        let key = derive_key(password, salt)?;
        self.encryption_key = Some(key);
        Ok(())
    }

    /// Generate a new encryption salt
    pub fn generate_salt() -> Vec<u8> {
        use rand::RngCore;
        let mut salt = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut salt);
        salt
    }

    /// Check authentication
    pub(super) fn require_auth(&self) -> Result<&str, StorageError> {
        self.access_token
            .as_deref()
            .ok_or(StorageError::AuthRequired)
    }

    /// Check encryption key
    pub(super) fn require_encryption_key(&self) -> Result<&[u8], StorageError> {
        self.encryption_key
            .as_deref()
            .ok_or(StorageError::Encryption("No encryption key set".to_string()))
    }
}
