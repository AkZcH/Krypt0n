use krypton::{
    KryptonError,
    engine::{decrypt, encrypt},
};

#[test]
fn decrypt_error_surface_collapsed_for_wrong_password() {
    let password = b"correct-password";
    let wrong_password = b"wrong-password";

    let ciphertext = encrypt(password, b"secret-data", b"krypton-test").unwrap();

    let result = decrypt(wrong_password, &ciphertext);

    assert!(matches!(result, Err(KryptonError::DecryptionFailed)));
}

#[test]
fn decrypt_error_surface_collapsed_for_tampered_ciphertext() {
    let password = b"correct-password";
    let mut ciphertext = encrypt(password, b"secret-data", b"krypton-test").unwrap();
    ciphertext[5] ^= 0xFF;

    let result = decrypt(password, &ciphertext);

    assert!(matches!(result, Err(KryptonError::DecryptionFailed)));
}
