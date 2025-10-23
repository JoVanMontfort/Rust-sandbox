use hex;
use rust_encryption::{CaesarCipher, SecureCipher, XorCipher};

fn main() {
    println!("=== Basic Ciphers Demo ===");

    // XOR Cipher Demo
    let xor_cipher = XorCipher::new(b"secretkey");
    let plaintext = b"Hello, World!";
    let encrypted = xor_cipher.encrypt(plaintext);
    let decrypted = xor_cipher.decrypt(&encrypted);

    println!("XOR Cipher:");
    println!("Original: {}", String::from_utf8_lossy(plaintext));
    println!("Encrypted: {:?}", encrypted);
    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted));
    println!();

    // Caesar Cipher Demo
    let caesar = CaesarCipher::new(3);
    let text = "Hello World";
    let caesar_encrypted = caesar.encrypt(text);
    let caesar_decrypted = caesar.decrypt(&caesar_encrypted);

    println!("Caesar Cipher:");
    println!("Original: {}", text);
    println!("Encrypted: {}", caesar_encrypted);
    println!("Decrypted: {}", caesar_decrypted);
    println!();

    println!("=== Secure Ciphers Demo ===");

    // Secure Cipher Demo
    let password = "my_strong_password";
    let salt = SecureCipher::generate_salt();

    let secure_cipher = SecureCipher::new(password, &salt);
    let secure_data = b"Sensitive information that needs protection!";

    match secure_cipher.encrypt(secure_data) {
        Ok(encrypted) => {
            println!("Secure Encryption:");
            println!("Original: {}", String::from_utf8_lossy(secure_data));
            println!("Encrypted (hex): {}", hex::encode(&encrypted));

            match secure_cipher.decrypt(&encrypted) {
                Ok(decrypted) => {
                    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted));
                }
                Err(e) => println!("Decryption failed: {}", e),
            }

            // Hash Demo
            let hash = SecureCipher::hash_data(secure_data);
            println!("SHA-256 Hash: {}", hash);
        }
        Err(e) => println!("Encryption failed: {}", e),
    }
}