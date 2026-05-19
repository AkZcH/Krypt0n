# Krypton - Password-Based File Encryption Toolkit

![CI](https://github.com/AkZcH/krypton/actions/workflows/ci.yml/badge.svg)
![Rust](https://img.shields.io/badge/Rust-2024-orange)
![Encryption](https://img.shields.io/badge/Encryption-XChaCha20--Poly1305-blue)
![KDF](https://img.shields.io/badge/KDF-Argon2id-green)
![Container](https://img.shields.io/badge/Container-ready-informational)

Krypton is a Rust-based encryption toolkit for protecting files with a password.
It provides a command-line interface, a reusable library core, a versioned binary
envelope format, Docker support, tests, and performance benchmarks.

The project is designed as a security engineering implementation of modern
password-based file encryption. It intentionally uses established cryptographic
libraries instead of custom cryptography.

---

## Table of Contents

1. [What Krypton Does](#1-what-krypton-does)
2. [Why This Project Exists](#2-why-this-project-exists)
3. [Cryptography Background](#3-cryptography-background)
4. [Project Architecture](#4-project-architecture)
5. [File Structure](#5-file-structure)
6. [Encryption Flow](#6-encryption-flow)
7. [Decryption Flow](#7-decryption-flow)
8. [Streaming Encryption](#8-streaming-encryption)
9. [Envelope Format](#9-envelope-format)
10. [Command-Line Usage](#10-command-line-usage)
11. [Docker Usage](#11-docker-usage)
12. [Testing and Benchmarks](#12-testing-and-benchmarks)
13. [Security Model](#13-security-model)
14. [Development Workflow](#14-development-workflow)
15. [Roadmap](#15-roadmap)

---

## 1. What Krypton Does

Krypton turns a normal file into an encrypted file that can only be opened again
with the correct password.

Example:

```text
secrets.txt  -->  secrets.txt.kry
```

Later, the encrypted file can be decrypted back into the original file:

```text
secrets.txt.kry  -->  secrets.txt
```

At a high level:

```text
                 password
                    |
                    v
normal file --> [ Krypton ] --> encrypted .kry file
                    |
                    v
          tamper detection metadata
```

Krypton is useful for:

- encrypting local files
- scripting file protection workflows
- demonstrating secure envelope design
- learning modern password-based encryption architecture
- running encryption inside containers or automation pipelines

---

## 2. Why This Project Exists

File encryption sounds simple, but doing it safely requires more than running a
cipher over bytes. A practical encryption tool needs to answer questions such as:

- How does a password become a strong encryption key?
- How is tampering detected?
- How can large files be encrypted without loading them fully into memory?
- How does the decryptor know which algorithm was used?
- How can the file format evolve without breaking older data?

Krypton answers these with:

| Requirement | Krypton's Approach |
| --- | --- |
| Password hardening | Argon2id key derivation |
| Encryption | XChaCha20-Poly1305 |
| Tamper detection | AEAD authentication tags |
| File format | Versioned binary envelope |
| Large files | Chunked streaming encryption |
| Automation | CLI flags and environment-variable passwords |
| Deployment | Multi-stage Docker image with non-root runtime user |

---

## 3. Cryptography Background

This section explains the core security concepts used by Krypton.

### Passwords Are Not Encryption Keys

A password like `correct horse battery staple` is human-readable text. Encryption
algorithms need fixed-size binary keys.

Krypton uses a key derivation function:

```text
password + random salt
        |
        v
     Argon2id
        |
        v
  32-byte encryption key
```

Argon2id is intentionally expensive. That cost is useful: it slows down attackers
who try to guess passwords offline after stealing encrypted files.

### What Is a Salt?

A salt is random data generated during encryption. Krypton stores the salt inside
the encrypted file because it is not secret.

The salt ensures that the same password does not always produce the same key:

```text
password + salt A --> key A
password + salt B --> key B
```

### What Is a Nonce?

A nonce is a number used once by the encryption algorithm. Krypton uses random
24-byte nonces with XChaCha20-Poly1305.

In streaming mode, every chunk gets its own nonce.

### What Is AEAD?

AEAD means "Authenticated Encryption with Associated Data."

It provides two things at the same time:

- confidentiality: attackers cannot read the plaintext
- integrity: attackers cannot modify the ciphertext undetected

Krypton uses XChaCha20-Poly1305, which produces:

```text
plaintext + key + nonce + aad
        |
        v
XChaCha20-Poly1305
        |
        v
ciphertext + authentication tag
```

If a single byte of the ciphertext, tag, or authenticated metadata is changed,
decryption fails.

---

## 4. Project Architecture

Krypton is separated into small layers. Each layer has a specific job.

```text
Command-line interface
        |
        v
Streaming file layer
        |
        v
Encryption engine
        |
        v
Crypto primitives
        |
        v
Envelope format
```

### Layer Responsibilities

| Layer | Files | Responsibility |
| --- | --- | --- |
| CLI | `src/main.rs` | Parse commands, resolve passwords, open files |
| Stream | `src/stream/` | Encrypt and decrypt large files in chunks |
| Engine | `src/engine.rs` | Orchestrate in-memory encryption/decryption |
| Crypto | `src/crypto/` | Key derivation and AEAD operations |
| Envelope | `src/envelope/` | Serialize and parse encrypted file metadata |
| Errors | `src/error.rs` | Shared error surface |

The command-line interface does not implement cryptography directly. It delegates
to the streaming layer, which delegates to the cryptographic core.

---

## 5. File Structure

```text
krypton/
|-- Cargo.toml                 # Rust package manifest
|-- Cargo.lock                 # Locked dependency versions
|-- Dockerfile                 # Multi-stage container build
|-- README.md                  # Project guide
|-- BENCHMARKS.md              # Benchmark summary
|
|-- Docs/
|   |-- architecture.md        # Architecture notes
|   |-- benchmarks.md          # Benchmark interpretation
|   |-- format.md              # Binary format specification
|   `-- security.md            # Threat model and security notes
|
|-- src/
|   |-- main.rs                # CLI entry point
|   |-- lib.rs                 # Library module exports
|   |-- engine.rs              # In-memory encryption engine
|   |-- error.rs               # Error types
|   |
|   |-- crypto/
|   |   |-- aead.rs            # XChaCha20-Poly1305 wrapper
|   |   |-- kdf.rs             # Argon2id key derivation
|   |   `-- mod.rs
|   |
|   |-- envelope/
|   |   |-- format.rs          # Magic bytes, version, IDs, structs
|   |   |-- parse.rs           # Envelope parser
|   |   `-- mod.rs             # Envelope serializer exports
|   |
|   `-- stream/
|       |-- encrypt.rs         # Chunked streaming encryption
|       |-- decrypt.rs         # Chunked streaming decryption
|       `-- mod.rs
|
|-- tests/
|   |-- crypto.rs              # Crypto primitive tests
|   |-- engine.rs              # Engine roundtrip tests
|   |-- envelope.rs            # Format serialization tests
|   |-- error_surface.rs       # Error behavior tests
|   `-- stream.rs              # Streaming tests
|
`-- benches/
    |-- engine.rs              # Engine benchmarks
    |-- kdf.rs                 # Argon2id benchmark
    |-- stream.rs              # Streaming benchmark
    `-- memory_profile.rs      # Whole-file vs streaming comparison
```

---

## 6. Encryption Flow

When you run:

```bash
krypton encrypt secrets.txt --password hunter2
```

Krypton follows this path:

```text
1. CLI parses command
2. CLI resolves password
3. Input file is opened with a buffered reader
4. Output file is created
5. Streaming encryption starts
6. Salt is generated
7. Password is transformed into a 32-byte key using Argon2id
8. Stream header is written
9. File is read in 64 KB chunks
10. Each chunk is encrypted and authenticated
11. Each encrypted chunk record is written
12. Sensitive buffers are zeroized where applicable
```

### Simplified Code Path

```text
src/main.rs
    run_encrypt()
        |
        v
src/stream/encrypt.rs
    encrypt_stream()
        |
        +--> crypto/kdf.rs
        |       derive_key()
        |
        +--> crypto/aead.rs
                encrypt()
```

### What Gets Written

Streaming encryption writes a stream header first:

```text
KRYSTRM
version
cipher id
kdf id
salt
```

Then it writes one record per chunk:

```text
nonce
ciphertext length
ciphertext
authentication tag
```

---

## 7. Decryption Flow

When you run:

```bash
krypton decrypt secrets.txt.kry --password hunter2
```

Krypton performs the reverse operation:

```text
1. CLI parses command
2. CLI resolves password
3. Encrypted file is opened
4. Output file is created
5. Stream header is read
6. Magic bytes and version are validated
7. Cipher and KDF identifiers are validated
8. Salt is read from the encrypted file
9. Password is transformed into the same 32-byte key
10. Each encrypted chunk record is read
11. Each chunk is authenticated and decrypted
12. Plaintext chunks are written to the output file
```

If the password is wrong, the file is corrupted, or the metadata is invalid,
decryption fails.

### Simplified Code Path

```text
src/main.rs
    run_decrypt()
        |
        v
src/stream/decrypt.rs
    decrypt_stream()
        |
        +--> crypto/kdf.rs
        |       derive_key()
        |
        +--> crypto/aead.rs
                decrypt()
```

---

## 8. Streaming Encryption

Krypton's CLI uses streaming encryption for file operations.

The chunk size is defined in `src/stream/encrypt.rs`:

```rust
const CHUNK_SIZE: usize = 64 * 1024;
```

That means the file is read in 64 KB pieces:

```text
large file
   |
   +--> chunk 1: 64 KB --> encrypt --> write record
   +--> chunk 2: 64 KB --> encrypt --> write record
   +--> chunk 3: 64 KB --> encrypt --> write record
   +--> final chunk     --> encrypt --> write record
```

### Why Streaming Matters

Without streaming, encrypting a 4 GB file would require loading that file into
memory before encryption.

With streaming, Krypton only needs a bounded file buffer:

```text
whole-file approach:
    memory grows with input size

streaming approach:
    file buffer stays around the chunk size
```

Krypton includes a benchmark that compares both approaches:

```bash
cargo bench --bench memory_profile
```

Sample result for a 32 MiB input:

```text
Whole-file load then encrypt: 122.25 ms to 129.07 ms, approx 128.00 MiB peak heap growth
Chunked stream encrypt: 67.755 ms to 70.633 ms, approx 19.00 MiB peak heap growth
```

The exact numbers depend on the machine, but the important lesson is stable:
whole-file encryption grows with file size; streaming keeps file-buffer memory
bounded.

---

## 9. Envelope Format

Krypton uses a binary envelope so encrypted data carries the metadata needed for
safe decryption.

There are two related formats:

| Format | Used By | Magic Bytes |
| --- | --- | --- |
| Standard envelope | In-memory engine API | `KRY1` |
| Streaming envelope | CLI file encryption | `KRYSTRM` |

### Standard Envelope

The standard in-memory envelope contains:

```text
MAGIC
VERSION
CIPHER_ID
KDF_ID
SALT
NONCE
AAD_LENGTH
AAD
CIPHERTEXT
TAG
```

### Streaming Envelope

The streaming file format contains:

```text
STREAM_MAGIC
VERSION
CIPHER_ID
KDF_ID
SALT
CHUNK_RECORD_1
CHUNK_RECORD_2
CHUNK_RECORD_3
...
```

Each chunk record contains:

```text
NONCE
CIPHERTEXT_LENGTH
CIPHERTEXT
TAG
```

### Why Store Metadata?

The envelope lets Krypton verify:

- this is a Krypton file
- the version is supported
- the cipher suite is supported
- the key derivation function is supported
- the encrypted payload has not been silently modified

For the full specification, see `Docs/format.md`.

---

## 10. Command-Line Usage

### Build

```bash
cargo build --release
```

The compiled binary will be available at:

```text
target/release/krypton
```

### Encrypt a File

```bash
cargo run -- encrypt secrets.txt --password hunter2
```

Or, after building:

```bash
krypton encrypt secrets.txt --password hunter2
```

Default output:

```text
secrets.txt.kry
```

### Decrypt a File

```bash
cargo run -- decrypt secrets.txt.kry --password hunter2
```

Or:

```bash
krypton decrypt secrets.txt.kry --password hunter2
```

Default output:

```text
secrets.txt
```

### Choose an Output Path

```bash
krypton encrypt secrets.txt --output backup.kry --password hunter2
```

```bash
krypton decrypt backup.kry --output recovered.txt --password hunter2
```

### Read Password from an Environment Variable

This is useful for scripts and CI jobs:

```bash
KRYP_PASS=hunter2 krypton encrypt secrets.txt --password-env KRYP_PASS
```

PowerShell example:

```powershell
$env:KRYP_PASS = "hunter2"
krypton encrypt secrets.txt --password-env KRYP_PASS
```

### Interactive Password Prompt

If no password flag is provided, Krypton prompts for one:

```bash
krypton encrypt secrets.txt
```

Note: the current prompt reads from standard input. It is functional, but it is
not yet a hidden terminal password prompt.

---

## 11. Docker Usage

Build the image:

```bash
docker build -t krypton .
```

Encrypt a file from the current directory:

```bash
docker run --rm -v ${PWD}:/data krypton encrypt /data/secrets.txt --password hunter2
```

Decrypt:

```bash
docker run --rm -v ${PWD}:/data krypton decrypt /data/secrets.txt.kry --password hunter2
```

Use an environment variable:

```bash
docker run --rm \
  -e KRYP_PASS=hunter2 \
  -v ${PWD}:/data \
  krypton encrypt /data/secrets.txt --password-env KRYP_PASS
```

The runtime image uses a non-root `krypton` user.

---

## 12. Testing and Benchmarks

### Run Tests

```bash
cargo test
```

The test suite covers:

- AEAD encryption/decryption behavior
- key derivation behavior
- envelope serialization and parsing
- engine roundtrips
- streaming roundtrips
- wrong-password failures
- tampering failures
- large-file streaming behavior

### Run All Benchmarks

```bash
cargo bench
```

### Run Specific Benchmarks

```bash
cargo bench --bench kdf
cargo bench --bench engine
cargo bench --bench stream
cargo bench --bench memory_profile
```

Criterion reports are generated under:

```text
target/criterion/report/index.html
```

### Benchmark Categories

| Benchmark | Purpose |
| --- | --- |
| `kdf` | Measures Argon2id key derivation cost |
| `engine` | Measures in-memory encrypt/decrypt operations |
| `stream` | Measures streaming encryption throughput |
| `memory_profile` | Compares whole-file loading vs chunked streaming |

---

## 13. Security Model

Krypton is designed to protect against:

- reading encrypted files without the password
- offline password guessing at high speed
- ciphertext tampering
- corrupted encrypted chunks
- unsupported or malformed envelope metadata
- accidental parsing of unrelated files

Krypton does not protect against:

- weak passwords
- malware on the machine running Krypton
- attackers who can read the password as it is typed
- compromised operating systems
- full memory disclosure attacks
- side-channel attacks

### Security Design Choices

| Area | Choice |
| --- | --- |
| KDF | Argon2id |
| Cipher | XChaCha20 |
| Authentication | Poly1305 |
| Mode | AEAD |
| Salt | Random 16 bytes |
| Nonce | Random 24 bytes |
| Streaming | Independent authenticated chunk records |
| Secret cleanup | `zeroize` for selected sensitive buffers |

### Important Disclaimer

Krypton is a security-focused engineering project, but it has not undergone a
formal third-party cryptographic audit. Do not treat it as audited production
cryptography without review.

For more details, see `Docs/security.md`.

---

## 14. Development Workflow

### Format

```bash
cargo fmt
```

### Lint

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Test

```bash
cargo test
```

### Build Release Binary

```bash
cargo build --release
```

### Suggested Pre-Commit Check

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo bench --bench memory_profile --no-run
```

---

## 15. Roadmap

Planned or natural next improvements:

- hidden password prompt support
- safer output-file handling to avoid accidental overwrite
- stronger memory zeroization coverage
- fuzz testing for envelope parsers
- key-file or raw-key encryption mode
- structured CLI errors instead of panics
- optional compression before encryption
- hardware-backed key integration
- WASM-compatible encryption API
- formal cryptographic review preparation

---

## Summary

Krypton demonstrates how to build a modern password-based encryption tool with:

- password hardening through Argon2id
- authenticated encryption through XChaCha20-Poly1305
- tamper detection through authentication tags
- versioned metadata through a binary envelope format
- large-file support through chunked streaming
- reproducible usage through CLI, Docker, tests, and benchmarks

The simplest mental model is:

```text
password + file
      |
      v
Krypton derives a key, encrypts authenticated chunks, and writes a .kry file
      |
      v
only the correct password can authenticate and recover the original bytes
```
