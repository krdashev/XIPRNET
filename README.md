# XIPRNET

XIPRNET (eXtended Internet Protocol Router Network) — a defense-grade, open architecture secure messaging system.

- **MLS (Messaging Layer Security)** for scalable group E2EE with forward secrecy + post-compromise security.
- **HPKE** envelopes with hybrid post-quantum (ML-KEM + X25519).
- **OPAQUE aPAKE** for password authentication (server never sees passwords).
- **Hardware-bound keys** via Secure Enclave / StrongBox.
- **Sealed-sender routing** to minimize metadata leakage.
- **Key Transparency log** (CONIKS-style) to block silent MITM.
- **File transfer**: resumable uploads, per-object DEKs, S3/tus support.
- **Secure wipe**: RAM zeroization, sealed local storage, cryptographic erase.

## Architecture
/core/ # Rust crypto + protocol (MLS, HPKE, storage, zeroize)
/clients/ # iOS, Android, Desktop thin shells
/server/ # gateway, delivery queues, media store, key directory
/transparency/ # Merkle log + auditors
/infra/ # IaC, CI/CD, sigstore, SBOM

## Security Principles
- No server-side plaintext, ever.
- All crypto FIPS 140-3 validated where possible.
- PQ-ready design (hybrid today, pure PQ tomorrow).
- Device attestation enforced before decrypt.
- Wipe-on-demand destroys keys first, data second.

## Build
- **Rust stable** (core, servers).
- **Swift / Kotlin** (thin clients).
- **Docker + Terraform** (infra).
- **CI/CD**: reproducible builds, signed artifacts.

## Roadmap
1. MVP chat (MLS 1:1 + groups).
2. File transfer (resumable, E2EE).
3. Key transparency + sealed sender.
4. Enterprise posture: policy controls, attestation gating.
5. PQ default suites.

## Disclaimer
Prototype research system. Not audited, not for production.
