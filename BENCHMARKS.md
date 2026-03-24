# Krypton Benchmarks

This document records Criterion benchmark results for Krypton.

## How to Run

```bash
cargo bench
```

Criterion also generates an HTML report at `target/criterion/report/index.html`.

## Latest Results

Recorded on 2026-03-24 with:

```bash
cargo bench
```

Observed timings:

```text
Argon2id derive key: 47.288 ms to 58.550 ms
Encrypt 1MB: 32.598 ms to 34.140 ms
Decrypt 1MB: 38.506 ms to 43.112 ms
Stream encrypt 10MB: 58.929 ms to 64.818 ms
```

## Notes

- Measure KDF cost separately from encryption and decryption throughput.
- Expect Argon2id to dominate total runtime compared with symmetric encryption.
- Include machine specs when publishing results so reviewers can compare fairly.
