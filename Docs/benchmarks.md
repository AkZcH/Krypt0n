# Krypton Benchmarks

Benchmarks executed using Criterion.rs on a development laptop.

## Argon2id Key Derivation

```
argon2 derive key ≈ 49 ms
```

Provides resistance against offline brute-force attacks while maintaining acceptable CLI responsiveness.

---

## File Encryption (1 MB)

```
encrypt 1MB ≈ 100 ms
decrypt 1MB ≈ 58 ms
```

Includes Argon2id key derivation cost.

Actual encryption throughput is significantly faster once the key is derived.

---

## Streaming Encryption (10 MB)

```
stream encrypt 10MB ≈ 72 ms
```

Equivalent throughput:

```
≈ 140 MB/sec
```

Streaming mode operates in constant memory and supports multi-GB files safely.

---

## Whole-File vs Chunked Streaming (32 MB)

Run with:

```
cargo bench --bench memory_profile
```

This benchmark compares two workflows:

- whole-file encryption, which reads the full input into a `Vec<u8>` before encrypting
- chunked streaming encryption, which reads and encrypts the file in 64 KB chunks

Sample result:

```
whole-file load then encrypt: 122-129 ms
chunked stream encrypt: 68-71 ms
whole-file peak heap growth: about 128 MiB
chunked stream peak heap growth: about 19 MiB
```

The peak heap numbers are approximate because they include library allocations such as Argon2id's working memory. The important signal is the shape: whole-file encryption grows with input size, while streaming keeps file-buffer memory bounded.

---

## Performance Interpretation

Most runtime cost comes from:

```
Argon2id key derivation
```

This is intentional and improves resistance against password-guessing attacks.

AEAD encryption itself is fast and scales linearly with file size.

---

## Benchmark Environment

Benchmarks executed using:

```
cargo bench
Criterion.rs
Rust stable toolchain
```

Results will vary depending on CPU and system configuration.
