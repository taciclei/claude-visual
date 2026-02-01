//! Cryptographic operations for cloud storage

use super::types::{EncryptedData, EncryptionAlgorithm, StorageError};

/// Derive encryption key from password using Argon2
pub(super) fn derive_key(password: &str, salt: &[u8]) -> Result<Vec<u8>, StorageError> {
    use argon2::{
        password_hash::{PasswordHasher, SaltString},
        Argon2,
    };

    let salt_string = SaltString::encode_b64(salt)
        .map_err(|e| StorageError::Encryption(format!("Invalid salt: {}", e)))?;

    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt_string)
        .map_err(|e| StorageError::Encryption(format!("Key derivation failed: {}", e)))?;

    let hash_output = hash
        .hash
        .ok_or_else(|| StorageError::Encryption("No hash output".to_string()))?;

    Ok(hash_output.as_bytes().to_vec())
}

/// Encrypt data using AES-256-GCM or ChaCha20-Poly1305
pub(super) fn encrypt_data(
    data: &[u8],
    key: &[u8],
    algorithm: EncryptionAlgorithm,
) -> Result<EncryptedData, StorageError> {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    use base64::{engine::general_purpose::STANDARD, Engine};
    use rand::RngCore;

    match algorithm {
        EncryptionAlgorithm::Aes256Gcm => {
            // Generate random IV
            let mut iv = [0u8; 12];
            rand::thread_rng().fill_bytes(&mut iv);

            // Create cipher
            let key_array: [u8; 32] = key
                .try_into()
                .map_err(|_| StorageError::Encryption("Invalid key length".to_string()))?;
            let cipher = Aes256Gcm::new(&key_array.into());

            // Encrypt
            let nonce = Nonce::from_slice(&iv);
            let ciphertext = cipher
                .encrypt(nonce, data)
                .map_err(|e| StorageError::Encryption(format!("Encryption failed: {}", e)))?;

            // Generate salt for metadata
            let mut salt = [0u8; 32];
            rand::thread_rng().fill_bytes(&mut salt);

            Ok(EncryptedData {
                ciphertext: STANDARD.encode(&ciphertext),
                iv: STANDARD.encode(&iv),
                salt: STANDARD.encode(&salt),
                algorithm,
                version: 1,
            })
        }
        EncryptionAlgorithm::ChaCha20Poly1305 => {
            use chacha20poly1305::{ChaCha20Poly1305, KeyInit as ChaChaKeyInit};
            use chacha20poly1305::aead::Aead as ChaChaAead;

            // Generate random nonce
            let mut nonce_bytes = [0u8; 12];
            rand::thread_rng().fill_bytes(&mut nonce_bytes);

            // Create cipher
            let key_array: [u8; 32] = key
                .try_into()
                .map_err(|_| StorageError::Encryption("Invalid key length".to_string()))?;
            let cipher = ChaCha20Poly1305::new(&key_array.into());

            // Encrypt
            let nonce = chacha20poly1305::Nonce::from_slice(&nonce_bytes);
            let ciphertext = cipher
                .encrypt(nonce, data)
                .map_err(|e| StorageError::Encryption(format!("Encryption failed: {}", e)))?;

            // Generate salt for metadata
            let mut salt = [0u8; 32];
            rand::thread_rng().fill_bytes(&mut salt);

            Ok(EncryptedData {
                ciphertext: STANDARD.encode(&ciphertext),
                iv: STANDARD.encode(&nonce_bytes),
                salt: STANDARD.encode(&salt),
                algorithm,
                version: 1,
            })
        }
    }
}

/// Decrypt data
pub(super) fn decrypt_data(encrypted: &EncryptedData, key: &[u8]) -> Result<Vec<u8>, StorageError> {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    use base64::{engine::general_purpose::STANDARD, Engine};

    let ciphertext = STANDARD
        .decode(&encrypted.ciphertext)
        .map_err(|e| StorageError::Decryption(format!("Invalid ciphertext: {}", e)))?;

    let iv = STANDARD
        .decode(&encrypted.iv)
        .map_err(|e| StorageError::Decryption(format!("Invalid IV: {}", e)))?;

    match encrypted.algorithm {
        EncryptionAlgorithm::Aes256Gcm => {
            let key_array: [u8; 32] = key
                .try_into()
                .map_err(|_| StorageError::Decryption("Invalid key length".to_string()))?;
            let cipher = Aes256Gcm::new(&key_array.into());

            let nonce = Nonce::from_slice(&iv);
            cipher
                .decrypt(nonce, ciphertext.as_ref())
                .map_err(|e| StorageError::Decryption(format!("Decryption failed: {}", e)))
        }
        EncryptionAlgorithm::ChaCha20Poly1305 => {
            use chacha20poly1305::{ChaCha20Poly1305, KeyInit as ChaChaKeyInit};
            use chacha20poly1305::aead::Aead as ChaChaAead;

            let key_array: [u8; 32] = key
                .try_into()
                .map_err(|_| StorageError::Decryption("Invalid key length".to_string()))?;
            let cipher = ChaCha20Poly1305::new(&key_array.into());

            let nonce = chacha20poly1305::Nonce::from_slice(&iv);
            cipher
                .decrypt(nonce, ciphertext.as_ref())
                .map_err(|e| StorageError::Decryption(format!("Decryption failed: {}", e)))
        }
    }
}

/// Calculate SHA-256 hash
pub(super) fn calculate_hash(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_calculation() {
        let data = b"test data";
        let hash = calculate_hash(data);

        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // SHA-256 = 32 bytes = 64 hex chars
    }
}
