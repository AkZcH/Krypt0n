use std::{
    fs::File,
    io::{Cursor, Read, Write},
};

use krypton::{
    error::KryptonError,
    stream::{decrypt::decrypt_stream, encrypt::encrypt_stream},
};
use tempfile::tempdir;

#[test]
fn stream_roundtrip_works_across_multiple_chunks() {
    let password = b"stream password";
    let aad = b"krypton-stream-test";
    let plaintext = vec![0x5Au8; 200_000];

    let mut encrypted = Vec::new();
    encrypt_stream(Cursor::new(&plaintext), &mut encrypted, password, aad).unwrap();

    let mut decrypted = Vec::new();
    decrypt_stream(Cursor::new(&encrypted), &mut decrypted, password, aad).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[test]
fn stream_wrong_password_fails() {
    let password = b"correct";
    let wrong = b"wrong";
    let aad = b"krypton-stream-test";
    let plaintext = vec![0x33u8; 96_000];

    let mut encrypted = Vec::new();
    encrypt_stream(Cursor::new(&plaintext), &mut encrypted, password, aad).unwrap();

    let err = decrypt_stream(Cursor::new(&encrypted), Vec::new(), wrong, aad).unwrap_err();
    assert!(matches!(err, KryptonError::DecryptionFailed));
}

#[test]
fn stream_tampered_header_fails() {
    let password = b"stream password";
    let aad = b"krypton-stream-test";
    let plaintext = b"hello";

    let mut encrypted = Vec::new();
    encrypt_stream(Cursor::new(plaintext), &mut encrypted, password, aad).unwrap();
    encrypted[0] ^= 0xFF;

    let err = decrypt_stream(Cursor::new(&encrypted), Vec::new(), password, aad).unwrap_err();
    assert!(matches!(err, KryptonError::DecryptionFailed));
}

#[test]
fn streaming_roundtrip_large_file() {
    let dir = tempdir().unwrap();

    let input_path = dir.path().join("input.bin");
    let encrypted_path = dir.path().join("encrypted.kry");
    let output_path = dir.path().join("output.bin");

    let password = b"strong-password";
    let aad = b"krypton-stream-test";

    let mut input_file = File::create(&input_path).unwrap();
    let chunk = vec![42u8; 1024 * 1024];

    for _ in 0..50 {
        input_file.write_all(&chunk).unwrap();
    }

    drop(input_file);

    encrypt_stream(
        File::open(&input_path).unwrap(),
        File::create(&encrypted_path).unwrap(),
        password,
        aad,
    )
    .unwrap();

    decrypt_stream(
        File::open(&encrypted_path).unwrap(),
        File::create(&output_path).unwrap(),
        password,
        aad,
    )
    .unwrap();

    let mut original = Vec::new();
    let mut recovered = Vec::new();

    File::open(&input_path)
        .unwrap()
        .read_to_end(&mut original)
        .unwrap();

    File::open(&output_path)
        .unwrap()
        .read_to_end(&mut recovered)
        .unwrap();

    assert_eq!(original, recovered);
}

#[test]
fn streaming_detects_tampering() {
    let dir = tempdir().unwrap();

    let input_path = dir.path().join("input.bin");
    let encrypted_path = dir.path().join("encrypted.kry");

    let password = b"password";
    let aad = b"krypton-stream-test";

    std::fs::write(&input_path, vec![7u8; 1024 * 1024]).unwrap();

    encrypt_stream(
        File::open(&input_path).unwrap(),
        File::create(&encrypted_path).unwrap(),
        password,
        aad,
    )
    .unwrap();

    let mut corrupted = std::fs::read(&encrypted_path).unwrap();
    corrupted[30] ^= 0xFF;
    std::fs::write(&encrypted_path, corrupted).unwrap();

    let result = decrypt_stream(
        File::open(&encrypted_path).unwrap(),
        std::io::sink(),
        password,
        aad,
    );

    assert!(result.is_err());
}
