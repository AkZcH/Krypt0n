use std::io::{Read, Write};

use rand_core::{OsRng, RngCore};

use crate::{
    crypto::{aead, kdf},
    envelope::{CipherSuite, Kdf, VERSION},
    error::KryptonError,
};

const MAGIC: &[u8; 7] = b"KRYSTRM";
const CHUNK_SIZE: usize = 64 * 1024;

pub fn encrypt_stream<R: Read, W: Write>(
    mut reader: R,
    mut writer: W,
    password: &[u8],
    aad: &[u8],
) -> Result<(), KryptonError> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let key = kdf::derive_key(password, &salt)?;

    writer.write_all(MAGIC)?;
    writer.write_all(&[VERSION])?;
    writer.write_all(&[CipherSuite::XChaCha20Poly1305 as u8])?;
    writer.write_all(&[Kdf::Argon2id as u8])?;
    writer.write_all(&salt)?;

    let mut buffer = vec![0u8; CHUNK_SIZE];

    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }

        let mut nonce = [0u8; 24];
        OsRng.fill_bytes(&mut nonce);

        let (ciphertext, tag) = aead::encrypt(&key, &nonce, aad, &buffer[..read])?;

        writer.write_all(&nonce)?;
        writer.write_all(&(ciphertext.len() as u32).to_le_bytes())?;
        writer.write_all(&ciphertext)?;
        writer.write_all(&tag)?;
    }

    writer.flush()?;

    Ok(())
}
