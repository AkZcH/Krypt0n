# Krypton Architecture

## Overview

Krypton is a password-based encryption toolkit written in Rust that provides authenticated encryption using modern cryptographic primitives and a versioned binary envelope format with streaming support.

The system is structured into independent layers:

```
CLI
 ↓
Engine
 ↓
Crypto primitives
 ↓
Envelope format
 ↓
Storage / transport
```

Each layer has a single responsibility and minimal trust surface.

---

# High-Level Encryption Flow

Encryption pipeline:

```
password
  ↓
Argon2id (KDF)
  ↓
32-byte symmetric key
  ↓
XChaCha20-Poly1305 (AEAD)
  ↓
Envelope serializer
  ↓
output file
```

Streaming mode:

```
password
  ↓
Argon2id
  ↓
derived key
  ↓
chunk encrypt loop
  ↓
authenticated chunk stream
  ↓
output file
```

---

# Module Layout

Krypton is organized into the following modules:

```
src/
 ├── crypto/
 │   ├── kdf.rs
 │   └── aead.rs
 │
 ├── envelope/
 │   ├── format.rs
 │   └── parse.rs
 │
 ├── stream/
 │   ├── encrypt.rs
 │   └── decrypt.rs
 │
 ├── engine.rs
 └── main.rs
```

Each module has a clearly defined boundary.

---

# Module Responsibilities

## crypto/

Provides cryptographic primitives.

Responsibilities:

- Argon2id key derivation
- XChaCha20-Poly1305 encryption
- authentication tag handling
- nonce-safe AEAD usage

This module never performs file I/O.

It operates purely on byte buffers.

---

## envelope/

Defines Krypton’s binary ciphertext format.

Responsibilities:

- version tagging
- cipher identification
- KDF identification
- salt storage
- nonce storage
- ciphertext packaging
- authentication tag placement
- format validation during parsing

This enables forward compatibility and algorithm agility.

---

## engine.rs

Core orchestration layer.

Responsibilities:

- password → key derivation
- nonce generation
- encryption coordination
- envelope assembly
- envelope parsing
- decrypt verification pipeline

This is the central cryptographic execution layer.

---

## stream/

Provides constant-memory file encryption support.

Responsibilities:

- chunk-based encryption
- per-chunk nonce generation
- per-chunk authentication tags
- streaming-safe parsing
- corruption isolation

Streaming mode prevents large-file memory exhaustion.

---

## main.rs (CLI layer)

Provides command-line interface functionality.

Responsibilities:

- argument parsing
- password sourcing
- file reading/writing
- environment-variable password support
- streaming pipeline invocation

The CLI layer never implements cryptographic logic.

It delegates all operations to the engine.

---

# Encryption Envelope Architecture

Standard encryption output contains:

```
magic bytes
version
cipher id
kdf id
salt
nonce
aad
ciphertext
authentication tag
```

Streaming encryption output contains:

```
stream magic
version
cipher id
kdf id
salt
chunk record 1
chunk record 2
chunk record 3
...
```

Each chunk record contains:

```
nonce
ciphertext length
ciphertext
authentication tag
```

This structure enables:

- tamper detection
- corruption isolation
- format validation before decryption
- streaming safety
- algorithm upgrade compatibility

---

# Streaming Encryption Design

Streaming mode operates as:

```
read chunk
 ↓
generate nonce
 ↓
encrypt chunk
 ↓
write chunk record
 ↓
repeat
```

Security guarantees:

- nonce uniqueness per chunk
- independent authentication per chunk
- safe truncation detection
- safe corruption detection

Streaming encryption allows processing files larger than system memory.

---

# Nonce Strategy

Nonce usage:

| Mode                 | Strategy               |
| -------------------- | ---------------------- |
| Standard encryption  | single random nonce    |
| Streaming encryption | random nonce per chunk |

XChaCha20 uses extended nonces to reduce misuse risk.

Nonce reuse is avoided by design.

---

# Error Handling Strategy

Krypton uses explicit failure signaling:

```
invalid format → reject
tampered ciphertext → reject
wrong password → reject
unknown cipher id → reject
unknown version → reject
```

Decryption never proceeds after envelope validation failure.

This prevents partial or unsafe plaintext recovery.

---

# Security Boundary Design

Layer trust model:

```
CLI
(untrusted input)

↓

Engine
(validation boundary)

↓

Envelope parser
(format verification boundary)

↓

Crypto primitives
(trusted execution layer)
```

Input validation occurs before cryptographic execution.

---

# Design Principles

Krypton follows these architectural constraints:

- no custom cryptographic primitives
- authenticated encryption only
- versioned ciphertext format
- algorithm agility support
- streaming-safe encryption
- constant-memory large-file handling
- container-compatible runtime behavior

These constraints ensure safe evolution of the encryption format.

---

# Future Architecture Extensions

Planned additions:

- key-based encryption API
- envelope fuzz-testing harness
- memory zeroization layer
- optional hardware-backed key integration
- REST encryption microservice wrapper
- WASM-compatible encryption runtime

Architecture has been designed to support these upgrades without breaking compatibility.
