pub const MAGIC: &[u8; 4] = b"KRY1";
pub const VERSION: u8 = 1;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherSuite {
    XChaCha20Poly1305 = 0x01,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kdf {
    Argon2id = 0x01,
}

#[derive(Debug)]
pub struct Envelope {
    pub version: u8,
    pub cipher: CipherSuite,
    pub kdf: Kdf,

    pub nonce: Vec<u8>,
    pub salt: Vec<u8>,
    pub aad: Vec<u8>,

    pub ciphertext: Vec<u8>,
    pub tag: Vec<u8>,
}
