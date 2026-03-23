# Security Policy

## Overview

Krypton is a modern password-based encryption toolkit written in Rust using **Argon2id** for key derivation and **XChaCha20-Poly1305** for authenticated encryption.

It is designed as a misuse-resistant encryption engine with a versioned binary envelope format and streaming encryption support.

Krypton intentionally avoids legacy cryptographic primitives such as AES-CBC and PBKDF2.

---

# Supported Versions

Currently supported:

| Version | Supported |
| ------- | --------- |
| 0.x     | ✅ Yes    |

Until a stable 1.0 release, the project is considered **security-focused but evolving**.

---

# Cryptographic Design

Krypton relies exclusively on well-established modern primitives:

| Component      | Algorithm                      |
| -------------- | ------------------------------ |
| Key Derivation | Argon2id                       |
| Encryption     | XChaCha20                      |
| Authentication | Poly1305                       |
| Mode           | AEAD                           |
| Streaming Mode | Chunk-authenticated AEAD       |
| Nonce Strategy | Random per message / per chunk |

No custom cryptographic primitives are implemented.

All cryptographic operations are delegated to audited RustCrypto crates.

---

# Security Goals

Krypton is designed to provide:

### Confidentiality

Encrypted data cannot be read without the correct password.

### Integrity

Any modification of ciphertext is detected during decryption.

### Authenticity (key-based)

Only holders of the correct password-derived key can decrypt messages successfully.

### Password Hardening

Argon2id protects against offline brute-force attacks.

### Streaming Safety

Large files are encrypted using independently authenticated chunks to prevent corruption propagation.

---

# Threat Model

Krypton protects against attackers who:

- obtain encrypted files
- modify ciphertext
- attempt offline password guessing
- attempt replay or truncation attacks on encrypted streams

Krypton assumes attackers **do not control the runtime environment**.

---

# Out of Scope Threats

The following threats are explicitly out of scope:

### Weak Passwords

Security depends on password strength.

Krypton cannot protect users choosing weak passwords.

---

### Compromised Host System

If the operating system is compromised:

- memory contents may leak
- passwords may be captured
- decrypted plaintext may be exposed

---

### Side-Channel Attacks

Timing attacks, cache attacks, and power-analysis attacks are outside the scope of this project.

---

### Memory Disclosure Attacks

Memory scraping or swap leakage protection is not guaranteed in the current release.

Future versions may include additional zeroization protections.

---

# Envelope Format Security

Each encrypted file contains:

- magic identifier
- version byte
- cipher identifier
- KDF identifier
- salt
- nonce(s)
- authenticated ciphertext
- authentication tag

This structure ensures:

- forward compatibility
- algorithm agility
- tamper detection
- format validation prior to decryption

---

# Streaming Encryption Guarantees

Streaming encryption uses:

- independent per-chunk nonces
- independent authentication tags
- chunk boundary verification

This ensures:

- safe multi-GB encryption
- corruption isolation
- truncation detection
- replay resistance within stream structure

---

# Nonce Strategy

Krypton uses randomly generated nonces per encryption operation and per streaming chunk.

Nonce reuse is avoided by design.

XChaCha20's extended nonce size significantly reduces misuse risk.

---

# Dependency Security

Krypton depends on:

- RustCrypto AEAD crates
- Argon2 reference implementation (RustCrypto)
- rand_core secure randomness

These libraries are widely used and actively maintained.

---

# Secure Usage Recommendations

Users should:

- choose strong passwords
- protect decrypted plaintext
- avoid storing passwords in plaintext scripts
- use environment-variable secrets for automation
- rotate encrypted files if compromise is suspected

---

# Reporting Vulnerabilities

If you discover a security issue, please report it privately.

Preferred disclosure method:

GitHub Security Advisory (recommended)

or open an issue labeled:

```
security
```

Please include:

- description of the issue
- reproduction steps
- expected vs observed behavior
- environment details

Responsible disclosure is appreciated.

---

# Security Roadmap

Planned improvements include:

- memory zeroization hardening
- envelope fuzz testing
- streaming format verification tests
- key-based encryption API
- optional hardware-backed key support
- formal cryptographic review preparation
- deterministic envelope validation harness

---

# Disclaimer

Krypton has **not yet undergone formal third-party cryptographic audit**.

It is intended as a security engineering project demonstrating modern cryptographic design practices.
