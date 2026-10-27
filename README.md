# AMNESIAX

**AMNESIAX is an early-stage, open-source privacy and defensive-security project.** The first milestone is a small, testable security core—not a polished desktop application and not a claim that the host operating system is already hardened.

## Current scope

- Explicit, fail-closed policy decisions for a required network guard.
- Capability reporting that distinguishes supported, unsupported, and unknown states.
- Regression tests for safety-critical policy decisions.
- A written threat model and security reporting process.
- Continuous integration for formatting, linting, and tests.

The Rust crate is a **policy model only**. It does not configure a firewall, create a VPN/Tor tunnel, isolate applications, change MAC addresses, manage virtual machines, or prevent IP leaks. Those features require separately implemented and reviewed OS-specific adapters and integration tests.

## Design principles

1. **Fail closed:** if a required protection is inactive, degraded, or unknown, deny the protected action.
2. **Never overstate protection:** unknown or unsupported capabilities must not be shown as enabled.
3. **Local-first:** no account, telemetry, or remote service is required by the security core.
4. **Use established cryptography:** do not invent algorithms or protocols.
5. **Least privilege:** privileged operations belong in narrow, auditable platform adapters.
6. **Test before claiming:** unit tests are not proof of end-to-end host security.
7. **Preserve trustworthy diagnostics:** no covert log deletion or anti-forensic behavior.

## Repository layout

- crates/amnesiax-core/ — small, dependency-free Rust policy core.
- docs/THREAT_MODEL.md — initial scope and attacker assumptions.
- SECURITY.md — responsible vulnerability reporting.
- .github/workflows/ci.yml — formatting, lint, and test checks.

## Development

Install a current stable Rust toolchain, then run cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, and cargo test --workspace.

## Status

This repository is an initial scaffold. No security feature should be treated as operational until its platform implementation, permissions, failure modes, and end-to-end tests have been reviewed.

See docs/THREAT_MODEL.md and SECURITY.md.
