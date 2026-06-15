# QualiaDB Cryptography Status — for Webizen-Browser Agents

> Updated 2026-06-15 (qualia-core-db 0.0.12). This is a pointer/summary; the **canonical,
> detailed source of truth** lives in the qualiaDB repo:
> `C:\Projects\qualiaDB\docs\CRYPTO_STATUS_2026-06-15.md` (and
> `C:\Projects\qualiaDB\CRYPTO_IMPLEMENTATION_PLAN.md`). Read those before doing crypto work.

## Why this exists

Webizen-browser consumes QualiaDB's crypto via its WASM + native builds. Earlier audit/plan
docs in this repo (e.g. `QUALIA_DB_LOGIC_AUDIT.md`) described capabilities that were
**aspirational** at the time. As of qualia-core-db 0.0.12 the picture is concrete:

## What is REAL now (rely on it)

- **Post-quantum signatures: ML-DSA-65 (FIPS-204)** via the `fips204` crate — real, WASM-safe.
  Previously this was a SHA3 *simulation*; that fake path has been removed. (pk 1952 B,
  sk 4032 B, sig 3309 B.)
- **Ed25519** sign/verify (WAL, identities, non-PQ keys).
- **AEAD:** AES-256-GCM, ChaCha20-Poly1305, XChaCha20-Poly1305.
- **Hashing:** SHA-256, SHA-512, **BLAKE3**.
- **KDF:** HKDF-SHA256.

## What is NOT real yet (do not rely on)

- zk-SNARK/STARK "proofs" — scaffolding only (SHA-256 commitments, not real proofs).
- Kyber / NTRU / SPHINCS / RSA / ECDSA — enum variants with no backend.
- ML-DSA **VC-issuance wiring + multi-Quin storage** of large signatures — the signing
  primitive is ready; the credential-graph anchoring is still TODO.

## Gotchas for browser/desktop work

- All of the above **compiles to `wasm32-unknown-unknown`** (entropy via `getrandom` `js`).
- **DID signing is NOT in the SPARQL query layer** — `SparqlDidHandler::sign_with_did` fails
  closed (no keys there). Sign in the identity/key-vault layer.
- AEAD `decrypt_data` does not re-supply AAD (use `None` AAD for round-trips).

## When updating docs in THIS repo

If you touch any `WEBIZEN_*.md`, `QUALIA_DB_LOGIC_AUDIT.md`, `PROJECT_REVIEW.md`, or master
plan that mentions crypto, align the wording with the REAL/NOT-real lists above and link to
`qualiaDB/docs/CRYPTO_STATUS_2026-06-15.md` rather than restating details (which drift).
`QUALIA_DB_LOGIC_AUDIT.md` line ~31 has already been corrected as the reference example.
