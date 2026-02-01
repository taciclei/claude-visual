//! Encrypted Cloud Storage Module
//!
//! Provides encrypted storage for conversations and user data in the cloud.

mod client;
mod crypto;
mod types;

// Re-export public API
pub use client::CloudStorage;
pub use types::{
    EncryptedData, EncryptionAlgorithm, StorageError, StorageItemType, StorageMetadata,
    StorageQuota,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quota_calculations() {
        let quota = StorageQuota {
            total_bytes: 1000,
            used_bytes: 250,
            item_count: 10,
            max_items: 100,
        };

        assert_eq!(quota.available_bytes(), 750);
        assert!((quota.usage_percent() - 25.0).abs() < 0.01);
        assert!(!quota.is_exceeded());
    }

    #[test]
    fn test_quota_exceeded() {
        let quota = StorageQuota {
            total_bytes: 1000,
            used_bytes: 1000,
            item_count: 100,
            max_items: 100,
        };

        assert!(quota.is_exceeded());
    }
}
