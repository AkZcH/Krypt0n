# Krypton Benchmarks

This document records Criterion benchmark results for Krypton.

## How to Run

```bash
cargo bench
```

Criterion also generates an HTML report at `target/criterion/report/index.html`.

To run only the whole-file-vs-streaming comparison:

```bash
cargo bench --bench memory_profile
```

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

Memory-profile benchmark sample for a 32 MiB input:

```text
Whole-file load then encrypt: 122.25 ms to 129.07 ms, approx 128.00 MiB peak heap growth
Chunked stream encrypt: 67.755 ms to 70.633 ms, approx 19.00 MiB peak heap growth
```

## Notes

- Measure KDF cost separately from encryption and decryption throughput.
- Expect Argon2id to dominate total runtime compared with symmetric encryption.
- Include machine specs when publishing results so reviewers can compare fairly.
- The memory-profile benchmark uses an intentionally whole-file workflow as a comparison point; production CLI file encryption uses the chunked streaming path.
