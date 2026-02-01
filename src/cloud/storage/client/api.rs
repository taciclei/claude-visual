//! HTTP API operations for cloud storage

use super::super::crypto::{calculate_hash, decrypt_data, encrypt_data};
use super::super::types::{
    DownloadResponse, StorageError, StorageItemType, StorageMetadata, StorageQuota,
    UpdateRequest, UploadRequest,
};
use super::core::CloudStorage;
use std::collections::HashMap;

impl CloudStorage {
    /// Get storage quota
    pub async fn get_quota(&self) -> Result<StorageQuota, StorageError> {
        let token = self.require_auth()?;

        let response = self
            .client
            .get(format!("{}/api/storage/quota", self.base_url))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| StorageError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(StorageError::Server(format!(
                "Failed to get quota: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| StorageError::InvalidData(e.to_string()))
    }

    /// List items by type
    pub async fn list_items(
        &self,
        item_type: StorageItemType,
    ) -> Result<Vec<StorageMetadata>, StorageError> {
        let token = self.require_auth()?;

        let response = self
            .client
            .get(format!("{}/api/storage/items", self.base_url))
            .header("Authorization", format!("Bearer {}", token))
            .query(&[("type", format!("{:?}", item_type))])
            .send()
            .await
            .map_err(|e| StorageError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(StorageError::Server(format!(
                "Failed to list items: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| StorageError::InvalidData(e.to_string()))
    }

    /// Upload encrypted data
    pub async fn upload(
        &self,
        item_type: StorageItemType,
        data: &[u8],
        tags: HashMap<String, String>,
    ) -> Result<StorageMetadata, StorageError> {
        let token = self.require_auth()?;
        let key = self.require_encryption_key()?;

        // Encrypt data
        let encrypted = encrypt_data(data, key, self.algorithm)?;

        // Calculate content hash
        let content_hash = calculate_hash(data);

        let request = UploadRequest {
            item_type,
            encrypted,
            content_hash,
            tags,
        };

        let response = self
            .client
            .post(format!("{}/api/storage/upload", self.base_url))
            .header("Authorization", format!("Bearer {}", token))
            .json(&request)
            .send()
            .await
            .map_err(|e| StorageError::Network(e.to_string()))?;

        if response.status() == reqwest::StatusCode::CONFLICT {
            return Err(StorageError::Conflict("Version conflict".to_string()));
        }

        if response.status() == reqwest::StatusCode::INSUFFICIENT_STORAGE {
            return Err(StorageError::QuotaExceeded);
        }

        if !response.status().is_success() {
            return Err(StorageError::Server(format!(
                "Upload failed: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| StorageError::InvalidData(e.to_string()))
    }

    /// Download and decrypt data
    pub async fn download(&self, item_id: &str) -> Result<Vec<u8>, StorageError> {
        let token = self.require_auth()?;
        let key = self.require_encryption_key()?;

        let response = self
            .client
            .get(format!("{}/api/storage/items/{}", self.base_url, item_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| StorageError::Network(e.to_string()))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(StorageError::NotFound(item_id.to_string()));
        }

        if !response.status().is_success() {
            return Err(StorageError::Server(format!(
                "Download failed: {}",
                response.status()
            )));
        }

        let download: DownloadResponse = response
            .json()
            .await
            .map_err(|e| StorageError::InvalidData(e.to_string()))?;

        // Decrypt data
        let decrypted = decrypt_data(&download.encrypted, key)?;

        // Verify hash
        let hash = calculate_hash(&decrypted);
        if hash != download.content_hash {
            return Err(StorageError::InvalidData(
                "Content hash mismatch".to_string(),
            ));
        }

        Ok(decrypted)
    }

    /// Update existing item
    pub async fn update(
        &self,
        item_id: &str,
        data: &[u8],
        expected_version: u64,
    ) -> Result<StorageMetadata, StorageError> {
        let token = self.require_auth()?;
        let key = self.require_encryption_key()?;

        // Encrypt data
        let encrypted = encrypt_data(data, key, self.algorithm)?;
        let content_hash = calculate_hash(data);

        let request = UpdateRequest {
            encrypted,
            content_hash,
            expected_version,
        };

        let response = self
            .client
            .put(format!("{}/api/storage/items/{}", self.base_url, item_id))
            .header("Authorization", format!("Bearer {}", token))
            .json(&request)
            .send()
            .await
            .map_err(|e| StorageError::Network(e.to_string()))?;

        if response.status() == reqwest::StatusCode::CONFLICT {
            return Err(StorageError::Conflict("Version conflict".to_string()));
        }

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(StorageError::NotFound(item_id.to_string()));
        }

        if !response.status().is_success() {
            return Err(StorageError::Server(format!(
                "Update failed: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| StorageError::InvalidData(e.to_string()))
    }

    /// Delete item
    pub async fn delete(&self, item_id: &str) -> Result<(), StorageError> {
        let token = self.require_auth()?;

        let response = self
            .client
            .delete(format!("{}/api/storage/items/{}", self.base_url, item_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| StorageError::Network(e.to_string()))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(StorageError::NotFound(item_id.to_string()));
        }

        if !response.status().is_success() {
            return Err(StorageError::Server(format!(
                "Delete failed: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Get item metadata without downloading content
    pub async fn get_metadata(&self, item_id: &str) -> Result<StorageMetadata, StorageError> {
        let token = self.require_auth()?;

        let response = self
            .client
            .get(format!(
                "{}/api/storage/items/{}/metadata",
                self.base_url, item_id
            ))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| StorageError::Network(e.to_string()))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(StorageError::NotFound(item_id.to_string()));
        }

        if !response.status().is_success() {
            return Err(StorageError::Server(format!(
                "Metadata fetch failed: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| StorageError::InvalidData(e.to_string()))
    }
}
