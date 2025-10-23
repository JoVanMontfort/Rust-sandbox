/// Simple XOR cipher implementation
/// WARNING: This is for educational purposes only - not cryptographically secure!
pub struct XorCipher {
    key: Vec<u8>,
}

impl XorCipher {
    /// Create a new XorCipher with the given key
    pub fn new(key: &[u8]) -> Self {
        Self { key: key.to_vec() }
    }

    /// Encrypt data using XOR cipher
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        self.xor_operation(data)
    }

    /// Decrypt data using XOR cipher
    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        self.xor_operation(data)
    }

    /// XOR operation (same for encryption and decryption)
    fn xor_operation(&self, data: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ self.key[i % self.key.len()])
            .collect()
    }
}

/// Caesar cipher implementation
pub struct CaesarCipher {
    shift: u8,
}

impl CaesarCipher {
    pub fn new(shift: u8) -> Self {
        Self { shift }
    }

    pub fn encrypt(&self, text: &str) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let shifted = (c as u8 - base + self.shift) % 26;
                    (base + shifted) as char
                } else {
                    c
                }
            })
            .collect()
    }

    pub fn decrypt(&self, text: &str) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let shifted = (c as u8 - base + 26 - self.shift) % 26;
                    (base + shifted) as char
                } else {
                    c
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_cipher() {
        let cipher = XorCipher::new(b"key");
        let data = b"test data";
        let encrypted = cipher.encrypt(data);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(data, decrypted.as_slice());
    }

    #[test]
    fn test_caesar_cipher() {
        let cipher = CaesarCipher::new(5);
        let text = "Hello World";
        let encrypted = cipher.encrypt(text);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(text, decrypted);
    }

    #[test]
    fn test_caesar_wrap_around() {
        let cipher = CaesarCipher::new(13);
        let text = "xyzXYZ";
        let encrypted = cipher.encrypt(text);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(text, decrypted);
    }
}