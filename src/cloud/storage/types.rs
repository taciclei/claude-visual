//! Type definitions for cloud storage

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Cloud storage error types
#[derive(Debug, Error)]
pub enum StorageError {
    /// Network error
    #[error("Network error: {0}")]
    Network(String),
    /// Authentication required
    #[error("Authentication required")]
    AuthRequired,
    /// Encryption error
    #[error("Encryption error: {0}")]
    Encryption(String),
    /// Decryption error
    #[error("Decryption error: {0}")]
    Decryption(String),
    /// Not found
    #[error("Resource not found: {0}")]
    NotFound(String),
    /// Conflict
    #[error("Conflict: {0}")]
    Conflict(String),
    /// Quota exceeded
    #[error("Storage quota exceeded")]
    QuotaExceeded,
    /// Invalid data
    #[error("Invalid data: {0}")]
    InvalidData(String),
    /// Server error
    #[error("Server error: {0}")]
    Server(String),
}

/// Encrypted data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    /// Encrypted payload (base64)
    pub ciphertext: String,
    /// Initialization vector (base64)
    pub iv: String,
    /// Salt for key derivation (base64)
    pub salt: String,
    /// Encryption algorithm used
    pub algorithm: EncryptionAlgorithm,
    /// Version of encryption scheme
    pub version: u32,
}

/// Encryption algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    /// AES-256-GCM
    Aes256Gcm,
    /// ChaCha20-Poly1305
    ChaCha20Poly1305,
}

impl Default for EncryptionAlgorithm {
    fn default() -> Self {
        EncryptionAlgorithm::Aes256Gcm
    }
}

/// Storage item metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetadata {
    /// Item ID
    pub id: String,
    /// Item type
    pub item_type: StorageItemType,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modified timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Size in bytes
    pub size: u64,
    /// Content hash for integrity
    pub content_hash: String,
    /// Version number for conflict resolution
    pub version: u64,
    /// Custom tags
    pub tags: HashMap<String, String>,
}

/// Storage item types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageItemType {
    /// Conversation
    Conversation,
    /// Message
    Message,
    /// Project settings
    ProjectSettings,
    /// User preferences
    UserPreferences,
    /// Attachment
    Attachment,
}

/// Cloud storage quota
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageQuota {
    /// Total storage in bytes
    pub total_bytes: u64,
    /// Used storage in bytes
    pub used_bytes: u64,
    /// Item count
    pub item_count: u64,
    /// Max item count
    pub max_items: u64,
}

impl StorageQuota {
    /// Get available storage in bytes
    pub fn available_bytes(&self) -> u64 {
        self.total_bytes.saturating_sub(self.used_bytes)
    }

    /// Get usage percentage
    pub fn usage_percent(&self) -> f64 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.used_bytes as f64 / self.total_bytes as f64) * 100.0
        }
    }

    /// Check if quota is exceeded
    pub fn is_exceeded(&self) -> bool {
        self.used_bytes >= self.total_bytes || self.item_count >= self.max_items
    }
}

/// Upload request
#[derive(Debug, Serialize)]
pub(super) struct UploadRequest {
    pub(super) item_type: StorageItemType,
    pub(super) encrypted: EncryptedData,
    pub(super) content_hash: String,
    pub(super) tags: HashMap<String, String>,
}

/// Update request
#[derive(Debug, Serialize)]
pub(super) struct UpdateRequest {
    pub(super) encrypted: EncryptedData,
    pub(super) content_hash: String,
    pub(super) expected_version: u64,
}

/// Download response
#[derive(Debug, Deserialize)]
pub(super) struct DownloadResponse {
    pub(super) encrypted: EncryptedData,
    pub(super) content_hash: String,
}
