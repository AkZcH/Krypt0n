use std::io::Cursor;

use criterion::{Criterion, criterion_group, criterion_main};
use krypton::stream::encrypt::encrypt_stream;

fn bench_stream_encrypt(c: &mut Criterion) {
    let password = b"stream-password";
    let data = vec![1u8; 10 * 1024 * 1024];

    c.bench_function("stream encrypt 10MB", |b| {
        b.iter(|| {
            encrypt_stream(
                Cursor::new(&data),
                std::io::sink(),
                password,
                b"krypton-stream",
            )
            .unwrap()
        })
    });
}

criterion_group!(benches, bench_stream_encrypt);
criterion_main!(benches);
