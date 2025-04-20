use chacha20poly1305::{Key, KeyInit, XChaCha20Poly1305, XNonce, aead::Aead};
use rand::RngCore;
use rand_core::OsRng;

pub fn encrypt(plaintext: &[u8], key_bytes: &[u8; 32]) -> (Vec<u8>, [u8; 24]) {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key_bytes));
    let mut nonce_bytes = [0u8; 24];
    OsRng.fill_bytes(&mut nonce_bytes);

    let nonce = XNonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, plaintext).expect("encryption failed");
    (ciphertext, nonce_bytes)
}

pub fn decrypt(ciphertext: &[u8], nonce_bytes: &[u8; 24], key_bytes: &[u8; 32]) -> Vec<u8> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key_bytes));
    let nonce = XNonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .expect("decryption failed")
}
