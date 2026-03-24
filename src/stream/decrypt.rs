use std::io::{ErrorKind, Read, Write};
use zeroize::Zeroize;

use crate::{
    crypto::{aead, kdf},
    envelope::{CipherSuite, Kdf, VERSION},
    error::KryptonError,
};

const MAGIC: &[u8; 7] = b"KRYSTRM";
const NONCE_LEN: usize = 24;
const TAG_LEN: usize = 16;

pub fn decrypt_stream<R: Read, W: Write>(
    mut reader: R,
    mut writer: W,
    password: &[u8],
    aad: &[u8],
) -> Result<(), KryptonError> {
    let mut header = [0u8; 7];
    reader
        .read_exact(&mut header)
        .map_err(|_| KryptonError::DecryptionFailed)?;
    if &header != MAGIC {
        return Err(KryptonError::DecryptionFailed);
    }

    let mut version = [0u8; 1];
    reader
        .read_exact(&mut version)
        .map_err(|_| KryptonError::DecryptionFailed)?;
    if version[0] != VERSION {
        return Err(KryptonError::DecryptionFailed);
    }

    let mut cipher = [0u8; 1];
    reader
        .read_exact(&mut cipher)
        .map_err(|_| KryptonError::DecryptionFailed)?;
    if cipher[0] != CipherSuite::XChaCha20Poly1305 as u8 {
        return Err(KryptonError::DecryptionFailed);
    }

    let mut kdf_id = [0u8; 1];
    reader
        .read_exact(&mut kdf_id)
        .map_err(|_| KryptonError::DecryptionFailed)?;
    if kdf_id[0] != Kdf::Argon2id as u8 {
        return Err(KryptonError::DecryptionFailed);
    }

    let mut salt = [0u8; 16];
    reader
        .read_exact(&mut salt)
        .map_err(|_| KryptonError::DecryptionFailed)?;

    let mut key = kdf::derive_key(password, &salt).map_err(|_| KryptonError::DecryptionFailed)?;
    salt.zeroize();

    let result = (|| -> Result<(), KryptonError> {
        while let Some(mut nonce) = read_nonce(&mut reader)? {
            let mut len_buf = [0u8; 4];
            reader
                .read_exact(&mut len_buf)
                .map_err(|_| KryptonError::DecryptionFailed)?;
            let len = u32::from_le_bytes(len_buf) as usize;

            let mut ciphertext = vec![0u8; len];
            reader
                .read_exact(&mut ciphertext)
                .map_err(|_| KryptonError::DecryptionFailed)?;

            let mut tag = [0u8; TAG_LEN];
            reader
                .read_exact(&mut tag)
                .map_err(|_| KryptonError::DecryptionFailed)?;

            let mut plaintext = aead::decrypt(&key, &nonce, aad, &ciphertext, &tag)
                .map_err(|_| KryptonError::DecryptionFailed)?;
            writer.write_all(&plaintext)?;

            plaintext.zeroize();
            tag.zeroize();
            nonce.zeroize();
        }

        writer.flush()?;

        Ok(())
    })();
    key.zeroize();
    result
}

fn read_nonce<R: Read>(reader: &mut R) -> Result<Option<[u8; NONCE_LEN]>, KryptonError> {
    let mut nonce = [0u8; NONCE_LEN];

    match reader.read(&mut nonce[..1]) {
        Ok(0) => Ok(None),
        Ok(1) => {
            reader
                .read_exact(&mut nonce[1..])
                .map_err(|_| KryptonError::DecryptionFailed)?;
            Ok(Some(nonce))
        }
        Ok(_) => Err(KryptonError::DecryptionFailed),
        Err(err) if err.kind() == ErrorKind::UnexpectedEof => Ok(None),
        Err(_) => Err(KryptonError::DecryptionFailed),
    }
}
