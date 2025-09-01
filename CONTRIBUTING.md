# Contributing to XIPRNET

We accept contributions under strict review.

## Code Standards
- **Rust (core, server):**
  - `cargo fmt` enforced
  - `clippy` warnings = errors
  - `#![forbid(unsafe_code)]` unless explicitly approved
  - All cryptographic operations use approved crates (`ring`, `rust-crypto`, `openmls`)

- **Swift (iOS):**
  - SwiftLint mandatory
  - Use iOS Keychain + Secure Enclave APIs
  - No custom crypto in Swift layer

- **Kotlin (Android):**
  - Android Lint mandatory
  - Use Jetpack Security + Keystore/StrongBox APIs
  - No custom crypto in Kotlin layer

## Tests
- PRs must include unit + integration tests.
- New protocol code must include test vectors.

## Process
1. Fork and branch from `main`.
2. Make changes, run all checks: `cargo test`, `cargo audit`, `cargo deny`.
3. Submit PR with detailed description.
4. At least **2 reviewers** must approve (see CODEOWNERS).
5. CI must pass before merge.

## Commit Conventions
- Use [Conventional Commits](https://www.conventionalcommits.org).
- Example: `feat(core): add HPKE hybrid support`

## Documentation
- Update `/docs/` if protocol or architecture changes.
