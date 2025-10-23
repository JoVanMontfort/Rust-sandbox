pub mod basic_ciphers;
pub mod secure_ciphers;

// Re-export for easier access
pub use basic_ciphers::{XorCipher, CaesarCipher};
pub use secure_ciphers::SecureCipher;