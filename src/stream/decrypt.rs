use std::io::{ErrorKind, Read, Write};

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
    reader.read_exact(&mut header)?;
    if &header != MAGIC {
        return Err(KryptonError::InvalidEnvelope);
    }

    let mut version = [0u8; 1];
    reader.read_exact(&mut version)?;
    if version[0] != VERSION {
        return Err(KryptonError::Unsupported);
    }

    let mut cipher = [0u8; 1];
    reader.read_exact(&mut cipher)?;
    if cipher[0] != CipherSuite::XChaCha20Poly1305 as u8 {
        return Err(KryptonError::Unsupported);
    }

    let mut kdf_id = [0u8; 1];
    reader.read_exact(&mut kdf_id)?;
    if kdf_id[0] != Kdf::Argon2id as u8 {
        return Err(KryptonError::Unsupported);
    }

    let mut salt = [0u8; 16];
    reader.read_exact(&mut salt)?;

    let key = kdf::derive_key(password, &salt)?;

    while let Some(nonce) = read_nonce(&mut reader)? {
        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf)?;
        let len = u32::from_le_bytes(len_buf) as usize;

        let mut ciphertext = vec![0u8; len];
        reader.read_exact(&mut ciphertext)?;

        let mut tag = [0u8; TAG_LEN];
        reader.read_exact(&mut tag)?;

        let plaintext = aead::decrypt(&key, &nonce, aad, &ciphertext, &tag)?;
        writer.write_all(&plaintext)?;
    }

    writer.flush()?;

    Ok(())
}

fn read_nonce<R: Read>(reader: &mut R) -> Result<Option<[u8; NONCE_LEN]>, KryptonError> {
    let mut nonce = [0u8; NONCE_LEN];

    match reader.read(&mut nonce[..1]) {
        Ok(0) => Ok(None),
        Ok(1) => {
            reader.read_exact(&mut nonce[1..])?;
            Ok(Some(nonce))
        }
        Ok(_) => Err(KryptonError::InvalidEnvelope),
        Err(err) if err.kind() == ErrorKind::UnexpectedEof => Ok(None),
        Err(err) => Err(err.into()),
    }
}
