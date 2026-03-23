use chacha20poly1305::{
    Key, XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit, Payload},
};

use crate::error::KryptonError;

pub const TAG_LEN: usize = 16;

pub fn encrypt(
    key: &[u8; 32],
    nonce: &[u8],
    aad: &[u8],
    plaintext: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), KryptonError> {
    if nonce.len() != 24 {
        return Err(KryptonError::Internal);
    }

    let cipher = XChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = XNonce::from_slice(nonce);

    let ciphertext = cipher
        .encrypt(
            nonce,
            Payload {
                msg: plaintext,
                aad,
            },
        )
        .map_err(|_| KryptonError::EncryptionFailed)?;

    let ct_len = ciphertext.len() - TAG_LEN;
    let ct = ciphertext[..ct_len].to_vec();
    let tag = ciphertext[ct_len..].to_vec();

    Ok((ct, tag))
}

pub fn decrypt(
    key: &[u8; 32],
    nonce: &[u8],
    aad: &[u8],
    ciphertext: &[u8],
    tag: &[u8],
) -> Result<Vec<u8>, KryptonError> {
    if nonce.len() != 24 {
        return Err(KryptonError::Internal);
    }

    let cipher = XChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = XNonce::from_slice(nonce);

    let mut combined = Vec::with_capacity(ciphertext.len() + tag.len());
    combined.extend_from_slice(ciphertext);
    combined.extend_from_slice(tag);

    cipher
        .decrypt(
            nonce,
            Payload {
                msg: &combined,
                aad,
            },
        )
        .map_err(|_| KryptonError::DecryptionFailed)
}
