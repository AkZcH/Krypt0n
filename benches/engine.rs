use criterion::{Criterion, criterion_group, criterion_main};
use krypton::engine::{decrypt, encrypt};

fn bench_encrypt(c: &mut Criterion) {
    let password = b"benchmark-password";
    let plaintext = vec![42u8; 1024 * 1024];

    c.bench_function("encrypt 1MB", |b| {
        b.iter(|| encrypt(password, &plaintext, b"krypton-bench").unwrap())
    });
}

fn bench_decrypt(c: &mut Criterion) {
    let password = b"benchmark-password";
    let plaintext = vec![42u8; 1024 * 1024];
    let ciphertext = encrypt(password, &plaintext, b"krypton-bench").unwrap();

    c.bench_function("decrypt 1MB", |b| {
        b.iter(|| decrypt(password, &ciphertext).unwrap())
    });
}

criterion_group!(benches, bench_encrypt, bench_decrypt);
criterion_main!(benches);
