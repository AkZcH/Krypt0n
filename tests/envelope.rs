use krypton::envelope::*;
use krypton::error::KryptonError;

#[test]
fn envelope_roundtrip() {
    let env = Envelope {
        version: VERSION,
        cipher: CipherSuite::XChaCha20Poly1305,
        kdf: Kdf::Argon2id,
        nonce: vec![1, 2, 3],
        salt: vec![4, 5, 6],
        aad: b"metadata".to_vec(),
        ciphertext: b"secret data".to_vec(),
        tag: vec![0u8; 16],
    };

    let bytes = serialize(&env).unwrap();
    let parsed = parse(&bytes).unwrap();

    assert_eq!(parsed.nonce, env.nonce);
    assert_eq!(parsed.salt, env.salt);
    assert_eq!(parsed.aad, env.aad);
    assert_eq!(parsed.ciphertext, env.ciphertext);
    assert_eq!(parsed.tag, env.tag);
}

#[test]
fn tampered_magic_fails() {
    let data = vec![
        b'X', b'R', b'Y', b'1', // wrong magic
        VERSION, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00,
    ];

    let res = parse(&data);
    assert!(matches!(res, Err(KryptonError::InvalidEnvelope)));
}

#[test]
fn unknown_cipher_fails() {
    let mut data = Vec::new();
    data.extend_from_slice(MAGIC);
    data.push(VERSION);
    data.push(0xFF); // unknown cipher
    data.push(0x01);
    data.push(0x00);
    data.push(0x00);
    data.extend_from_slice(&0u16.to_be_bytes());
    data.extend_from_slice(&[0u8; 16]);

    let res = parse(&data);
    assert!(matches!(res, Err(KryptonError::Unsupported)));
}
