use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2;
use rand::Rng;
use sha2::{Digest, Sha256};

pub struct SecureCipher {
    cipher: Aes256Gcm,
}

impl SecureCipher {
    /// Create a new SecureCipher with a key derived from a password
    pub fn new(password: &str, salt: &[u8]) -> Self {
        let mut key_bytes = [0u8; 32]; // 256-bit key
        pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, 100_000, &mut key_bytes);

        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        Self { cipher }
    }

    /// Generate a random salt
    pub fn generate_salt() -> Vec<u8> {
        let mut salt = [0u8; 16];
        rand::thread_rng().fill(&mut salt);
        salt.to_vec()
    }

    /// Encrypt data using AES-256-GCM
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        self.cipher
            .encrypt(&nonce, data)
            .map(|mut ciphertext| {
                let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
                result.extend_from_slice(&nonce);
                result.append(&mut ciphertext);
                result
            })
            .map_err(|e| format!("Encryption failed: {}", e))
    }

    /// Decrypt data using AES-256-GCM
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        if data.len() < 12 {
            return Err("Data too short".to_string());
        }

        let nonce = Nonce::from_slice(&data[..12]);
        let ciphertext = &data[12..];

        self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))
    }

    /// Create a cryptographic hash of data
    pub fn hash_data(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_cipher_encryption() {
        let password = "test_password";
        let salt = SecureCipher::generate_salt();
        let cipher = SecureCipher::new(password, &salt);
        let data = b"Hello, Secure World!";

        let encrypted = cipher.encrypt(data).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();

        assert_eq!(data, decrypted.as_slice());
    }

    #[test]
    fn test_hash_consistency() {
        let data = b"test data";
        let hash1 = SecureCipher::hash_data(data);
        let hash2 = SecureCipher::hash_data(data);

        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA-256 produces 64-character hex string
    }
}