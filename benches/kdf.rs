use criterion::{Criterion, criterion_group, criterion_main};
use krypton::crypto::kdf::derive_key;

fn bench_kdf(c: &mut Criterion) {
    let password = b"benchmark-password";
    let salt = [0u8; 16];

    c.bench_function("argon2 derive key", |b| {
        b.iter(|| derive_key(password, &salt).unwrap())
    });
}

criterion_group!(benches, bench_kdf);
criterion_main!(benches);
