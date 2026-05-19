# Krypton Encryption Format Specification

## Overview

Krypton uses a **versioned binary envelope format** for encrypted data.

This format provides:

* authenticated encryption
* forward compatibility
* algorithm agility
* corruption detection
* safe parsing before decryption
* streaming-safe chunk processing

Two envelope formats exist:

```
Standard encryption format
Streaming encryption format
```

---

# Standard Encryption Envelope

The standard encryption envelope is used for in-memory encryption operations and small-to-medium file encryption.

Structure:

```
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

---

# Field Definitions

## MAGIC

```
4 bytes
```

Identifies Krypton ciphertext.

Example:

```
"KRY1"
```

Used to prevent accidental parsing of unrelated files.

---

## VERSION

```
1 byte
```

Indicates envelope version.

Example:

```
0x01
```

Future versions may introduce:

* new cipher suites
* new metadata
* new streaming structures

---

## CIPHER_ID

```
1 byte
```

Identifies encryption algorithm.

Current value:

```
0x01 → XChaCha20-Poly1305
```

Future support may include:

```
AES-GCM
ChaCha20-Poly1305
post-quantum AEAD
```

---

## KDF_ID

```
1 byte
```

Identifies key-derivation function.

Current value:

```
0x01 → Argon2id
```

Allows future support for:

```
scrypt
Argon2d
Argon2i
hardware-backed KDF
```

---

## SALT

```
16 bytes
```

Randomly generated per encryption operation.

Used as input to Argon2id.

Ensures identical passwords produce different derived keys.

---

## NONCE

```
24 bytes
```

Randomly generated per encryption operation.

Used by XChaCha20-Poly1305.

Provides nonce-misuse resistance compared to 96-bit nonce AEAD schemes.

---

## AAD_LENGTH

```
2 bytes (little-endian)
```

Length of Additional Authenticated Data.

AAD is authenticated but not encrypted.

Used for:

```
format binding
context separation
protocol version tagging
```

---

## AAD

```
variable length
```

Authenticated metadata.

Example:

```
"krypton-cli:v1"
```

Tampering with AAD causes authentication failure.

---

## CIPHERTEXT

```
variable length
```

Encrypted payload output by AEAD algorithm.

Contains:

```
plaintext XOR keystream
```

Authenticated using Poly1305.

---

## TAG

```
16 bytes
```

Authentication tag produced by Poly1305.

Ensures:

```
integrity
authenticity
tamper detection
```

Any modification causes decryption failure.

---

# Streaming Encryption Format

Streaming mode supports encryption of arbitrarily large files.

Structure:

```
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

---

# STREAM_MAGIC

```
7 bytes
```

Value:

```
"KRYSTRM"
```

Distinguishes streaming ciphertext from standard envelope format.

---

# Chunk Record Structure

Each chunk record contains:

```
NONCE
CIPHERTEXT_LENGTH
CIPHERTEXT
TAG
```

---

## NONCE

```
24 bytes
```

Random per chunk.

Ensures nonce uniqueness across stream.

Prevents nonce reuse vulnerabilities.

---

## CIPHERTEXT_LENGTH

```
4 bytes (little-endian)
```

Length of encrypted chunk.

Used to:

```
parse stream safely
detect truncation
detect corruption
```

---

## CIPHERTEXT

```
variable length
```

Encrypted chunk payload.

Authenticated independently.

---

## TAG

```
16 bytes
```

Poly1305 authentication tag.

Provides:

```
per-chunk integrity verification
corruption isolation
safe truncation detection
```

---

# Streaming Security Guarantees

Streaming encryption provides:

```
constant memory usage
independent chunk authentication
safe corruption detection
safe truncation detection
nonce uniqueness per chunk
```

Corruption of one chunk does not affect other chunks.

---

# Parsing Strategy

Krypton parsing follows strict validation order:

```
verify MAGIC
verify VERSION
verify CIPHER_ID
verify KDF_ID
verify envelope length
verify authentication tag
decrypt payload
```

Decryption never proceeds if validation fails.

---

# Algorithm Agility

Envelope metadata enables future upgrades without breaking compatibility:

```
new cipher suites
new KDF implementations
new streaming formats
new metadata extensions
```

Older clients safely reject unsupported formats.

---

# Security Considerations

The envelope format ensures:

```
tamper detection
version awareness
algorithm identification
nonce uniqueness
salt uniqueness
authenticated metadata binding
```

It prevents:

```
format confusion attacks
cipher substitution attacks
silent corruption acceptance
```

---

# Future Format Extensions

Planned extensions include:

```
optional compression flag
multi-recipient encryption
key-based encryption mode
hardware-backed key identifiers
envelope signatures
```

Envelope versioning supports safe rollout of these features.
