use crate::error::KryptonError;
use argon2::Argon2;

pub fn derive_key(password: &[u8], salt: &[u8]) -> Result<[u8; 32], KryptonError> {
    let mut key = [0u8; 32];

    let argon2 = Argon2::default(); // Argon2id

    argon2
        .hash_password_into(password, salt, &mut key)
        .map_err(|_| KryptonError::Internal)?;

    Ok(key)
}
