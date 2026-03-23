use crate::envelope::format::{CipherSuite, Envelope, Kdf, MAGIC, VERSION};
use crate::error::KryptonError;

const TAG_LEN: usize = 16;

pub fn serialize(env: &Envelope) -> Result<Vec<u8>, KryptonError> {
    let mut out = Vec::new();

    out.extend_from_slice(MAGIC);
    out.push(env.version);
    out.push(env.cipher as u8);
    out.push(env.kdf as u8);

    if env.nonce.len() > u8::MAX as usize {
        return Err(KryptonError::InvalidEnvelope);
    }
    out.push(env.nonce.len() as u8);
    out.extend_from_slice(&env.nonce);

    if env.salt.len() > u8::MAX as usize {
        return Err(KryptonError::InvalidEnvelope);
    }
    out.push(env.salt.len() as u8);
    out.extend_from_slice(&env.salt);

    if env.aad.len() > u16::MAX as usize {
        return Err(KryptonError::InvalidEnvelope);
    }
    out.extend_from_slice(&(env.aad.len() as u16).to_be_bytes());
    out.extend_from_slice(&env.aad);

    out.extend_from_slice(&env.ciphertext);
    out.extend_from_slice(&env.tag);

    Ok(out)
}

pub fn parse(input: &[u8]) -> Result<Envelope, KryptonError> {
    let mut idx = 0;

    if input.len() < MAGIC.len() + 1 + 1 + 1 + 1 + 1 + 2 + TAG_LEN {
        return Err(KryptonError::InvalidEnvelope);
    }

    if &input[idx..idx + 4] != MAGIC {
        return Err(KryptonError::InvalidEnvelope);
    }
    idx += 4;

    let version = input[idx];
    if version != VERSION {
        return Err(KryptonError::Unsupported);
    }
    idx += 1;

    let cipher = match input[idx] {
        0x01 => CipherSuite::XChaCha20Poly1305,
        _ => return Err(KryptonError::Unsupported),
    };
    idx += 1;

    let kdf = match input[idx] {
        0x01 => Kdf::Argon2id,
        _ => return Err(KryptonError::Unsupported),
    };
    idx += 1;

    let nonce_len = input[idx] as usize;
    idx += 1;
    if idx + nonce_len > input.len() {
        return Err(KryptonError::InvalidEnvelope);
    }
    let nonce = input[idx..idx + nonce_len].to_vec();
    idx += nonce_len;

    let salt_len = input[idx] as usize;
    idx += 1;
    if idx + salt_len > input.len() {
        return Err(KryptonError::InvalidEnvelope);
    }
    let salt = input[idx..idx + salt_len].to_vec();
    idx += salt_len;

    if idx + 2 > input.len() {
        return Err(KryptonError::InvalidEnvelope);
    }
    let aad_len = u16::from_be_bytes([input[idx], input[idx + 1]]) as usize;
    idx += 2;

    if idx + aad_len > input.len() {
        return Err(KryptonError::InvalidEnvelope);
    }
    let aad = input[idx..idx + aad_len].to_vec();
    idx += aad_len;

    if input.len() < idx + TAG_LEN {
        return Err(KryptonError::InvalidEnvelope);
    }

    let ct_len = input.len() - idx - TAG_LEN;
    let ciphertext = input[idx..idx + ct_len].to_vec();
    idx += ct_len;

    let tag = input[idx..idx + TAG_LEN].to_vec();

    Ok(Envelope {
        version,
        cipher,
        kdf,
        nonce,
        salt,
        aad,
        ciphertext,
        tag,
    })
}
