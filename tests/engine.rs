use krypton::engine::{decrypt, encrypt};

#[test]
fn encrypt_decrypt_roundtrip() {
    let password = b"strong password";
    let plaintext = b"very secret data";
    let aad = b"context";

    let encrypted = encrypt(password, plaintext, aad).unwrap();
    let decrypted = decrypt(password, &encrypted).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[test]
fn wrong_password_fails() {
    let password = b"correct password";
    let wrong = b"wrong password";
    let plaintext = b"secret";
    let aad = b"meta";

    let encrypted = encrypt(password, plaintext, aad).unwrap();
    let res = decrypt(wrong, &encrypted);

    assert!(res.is_err());
}

#[test]
fn tampering_fails() {
    let password = b"password";
    let plaintext = b"data";
    let aad = b"meta";

    let mut encrypted = encrypt(password, plaintext, aad).unwrap();

    // Flip a bit
    encrypted[10] ^= 0xFF;

    let res = decrypt(password, &encrypted);
    assert!(res.is_err());
}
