# Krypton 🔐

![CI](https://github.com/<your-username>/krypton/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/github/license/<your-username>/krypton)
![Rust](https://img.shields.io/badge/rust-stable-orange)
![Docker](https://img.shields.io/badge/container-ready-blue)

**Krypton** is a modern password-based encryption toolkit written in Rust that implements **Argon2id key derivation** and **XChaCha20-Poly1305 authenticated encryption** with a versioned binary envelope format and streaming file support.

It is designed as a secure, reproducible, container-ready encryption engine suitable for CLI workflows, scripting environments, and infrastructure pipelines.

---

# Features

- Argon2id password-based key derivation
- XChaCha20-Poly1305 authenticated encryption (AEAD)
- Versioned encryption envelope format
- Streaming encryption (multi-GB safe)
- Tamper detection via authentication tags
- CLI interface for file encryption
- Docker container runtime support
- Non-root container execution
- Strict Clippy + Rustfmt enforcement
- GitHub Actions CI pipeline
- Environment-variable password support for automation

---

# Quick Start

Build locally:

```bash
cargo build --release
```

Encrypt a file:

```bash
cargo run -- encrypt secrets.txt --password hunter2
```

Decrypt a file:

```bash
cargo run -- decrypt secrets.txt.kry --password hunter2
```

---

# CLI Usage

Encrypt:

```bash
krypton encrypt secrets.txt --password hunter2
```

Decrypt:

```bash
krypton decrypt secrets.txt.kry --password hunter2
```

Environment-variable password support:

```bash
KRYP_PASS=hunter2 krypton encrypt secrets.txt --password-env KRYP_PASS
```

Output files are generated automatically:

```
secrets.txt → secrets.txt.kry
secrets.txt.kry → secrets.txt
```

---

# Streaming Encryption Support

Krypton supports chunk-authenticated streaming encryption.

This enables:

- encryption of multi-GB files
- constant memory usage
- pipeline compatibility
- container-safe execution

Each chunk is independently authenticated using AEAD.

---

# Docker Usage

Build container:

```bash
docker build -t krypton .
```

Encrypt:

```bash
docker run -v ${PWD}:/data krypton encrypt /data/secrets.txt --password hunter2
```

Decrypt:

```bash
docker run -v ${PWD}:/data krypton decrypt /data/secrets.txt.kry --password hunter2
```

Environment-variable password example:

```bash
docker run \
  -e KRYP_PASS=hunter2 \
  -v ${PWD}:/data \
  krypton encrypt /data/secrets.txt --password-env KRYP_PASS
```

The container runs as a **non-root user** for improved runtime security.

---

# Cryptographic Design

Krypton uses only modern, misuse-resistant primitives:

| Component      | Algorithm               |
| -------------- | ----------------------- |
| KDF            | Argon2id                |
| Cipher         | XChaCha20               |
| Authentication | Poly1305                |
| Mode           | AEAD                    |
| Envelope       | Versioned binary format |

No custom cryptography is implemented.

---

# Security Model

Krypton protects against:

- offline brute-force attacks (via Argon2id)
- ciphertext tampering
- nonce reuse risks
- unauthenticated decryption
- partial file corruption during streaming

Krypton does **not** protect against:

- weak passwords
- compromised host machines
- memory disclosure attacks
- side-channel attacks

See `SECURITY.md` for details.

---

# Encryption Envelope Overview

Each encrypted file contains:

```
magic bytes
version
cipher identifier
kdf identifier
salt
nonce(s)
aad
ciphertext
authentication tag
```

Streaming mode stores authenticated chunk records sequentially.

See `FORMAT.md` for full specification.

---

# Architecture

```
password
  ↓
Argon2id
  ↓
derived key
  ↓
XChaCha20-Poly1305
  ↓
envelope serializer
  ↓
output file
```

Module layout:

```
crypto/
engine/
envelope/
stream/
cli/
```

See `ARCHITECTURE.md` for details.

---

# CI Pipeline

GitHub Actions automatically verifies:

- formatting (`cargo fmt`)
- linting (`cargo clippy`)
- unit tests
- streaming encryption tests
- Docker image build

Every push is validated automatically.

---

# Roadmap

Planned improvements:

- key-based encryption API
- memory zeroization hardening
- streaming performance benchmarks
- fuzz testing for envelope parser
- WASM support
- optional REST microservice wrapper

---

# License

This project is licensed under the MIT License.

See `LICENSE` for details.

---

# Why Krypton Exists

Krypton is designed as a **modern Rust cryptography systems project** demonstrating:

- secure envelope design
- authenticated streaming encryption
- containerized CLI tooling
- reproducible builds via CI/CD
- infrastructure-ready distribution workflows

It intentionally avoids legacy primitives such as AES-CBC or PBKDF2.

---

# Disclaimer

Krypton is provided as a security engineering project and research-grade encryption tool. It has **not undergone formal third-party cryptographic audit**.
