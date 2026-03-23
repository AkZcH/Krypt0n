use krypton::crypto::aead::{TAG_LEN, decrypt, encrypt};
use krypton::crypto::kdf::derive_key;

#[test]
fn derive_key_is_deterministic() {
    let password = b"correct horse battery staple";
    let salt = b"unique_salt";

    let k1 = derive_key(password, salt).unwrap();
    let k2 = derive_key(password, salt).unwrap();

    assert_eq!(k1, k2);
}

#[test]
fn aead_roundtrip_works() {
    let key = [7u8; 32];
    let nonce = [9u8; 24]; // XChaCha nonce length
    let aad = b"metadata";
    let plaintext = b"top secret data";

    let (ct, tag) = encrypt(&key, &nonce, aad, plaintext).unwrap();
    assert_eq!(tag.len(), TAG_LEN);

    let recovered = decrypt(&key, &nonce, aad, &ct, &tag).unwrap();
    assert_eq!(recovered, plaintext);
}

#[test]
fn wrong_tag_fails() {
    let key = [1u8; 32];
    let nonce = [2u8; 24];
    let aad = b"metadata";
    let plaintext = b"attack at dawn";

    let (ct, mut tag) = encrypt(&key, &nonce, aad, plaintext).unwrap();

    // Flip one bit in tag
    tag[0] ^= 0xFF;

    let res = decrypt(&key, &nonce, aad, &ct, &tag);
    assert!(res.is_err());
}
