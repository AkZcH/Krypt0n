use thiserror::Error;

#[derive(Debug, Error)]
pub enum KryptonError {
    #[error("i/o failed")]
    Io(#[from] std::io::Error),

    #[error("encryption failed")]
    EncryptionFailed,

    #[error("decryption failed")]
    DecryptionFailed,

    #[error("invalid envelope format")]
    InvalidEnvelope,

    #[error("unsupported version or algorithm")]
    Unsupported,

    #[error("internal error")]
    Internal,
}
