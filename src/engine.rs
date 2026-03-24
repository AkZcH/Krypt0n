use rand_core::{OsRng, RngCore};
use zeroize::Zeroize;

use crate::{
    crypto::{aead, kdf},
    envelope::{CipherSuite, Envelope, Kdf, VERSION},
    error::KryptonError,
};

pub fn encrypt(password: &[u8], plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>, KryptonError> {
    // 1. Generate salt
    let mut salt = vec![0u8; 16];
    OsRng.fill_bytes(&mut salt);

    // 2. Derive key
    let mut key = kdf::derive_key(password, &salt)?;

    // 3. Generate nonce (24 bytes for XChaCha)
    let mut nonce = vec![0u8; 24];
    OsRng.fill_bytes(&mut nonce);

    // 4. Encrypt
    let encrypted = aead::encrypt(&key, &nonce, aad, plaintext);
    key.zeroize();
    let (ciphertext, tag) = encrypted?;

    // 5. Build envelope
    let env = Envelope {
        version: VERSION,
        cipher: CipherSuite::XChaCha20Poly1305,
        kdf: Kdf::Argon2id,
        nonce,
        salt,
        aad: aad.to_vec(),
        ciphertext,
        tag,
    };

    // 6. Serialize
    crate::envelope::serialize(&env)
}

pub fn decrypt(password: &[u8], envelope_bytes: &[u8]) -> Result<Vec<u8>, KryptonError> {
    // 1. Parse envelope
    let env = crate::envelope::parse(envelope_bytes)?;

    // 2. Validate algorithms
    if env.cipher != CipherSuite::XChaCha20Poly1305 {
        return Err(KryptonError::Unsupported);
    }
    if env.kdf != Kdf::Argon2id {
        return Err(KryptonError::Unsupported);
    }

    // 3. Derive key
    let mut key = kdf::derive_key(password, &env.salt)?;

    // 4. Decrypt
    let decrypted = aead::decrypt(&key, &env.nonce, &env.aad, &env.ciphertext, &env.tag);
    key.zeroize();
    decrypted
}
