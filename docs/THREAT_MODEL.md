# AMNESIAX initial threat model

**Status:** draft; must be reviewed before implementation claims are made.

## Security goals

- Make protection state explicit and avoid silently treating unknown state as safe.
- Provide a small, testable policy layer for higher-level components.
- Plan for platform-specific enforcement with least privilege.
- Reduce accidental exposure from misconfiguration and failed protection components.
- Keep security claims tied to reproducible tests and documented platform support.

## In scope for the first milestone

- Pure policy decisions and unit tests.
- Capability status representation.
- CI checks for Rust formatting, linting, and tests.
- Documented reporting and development practices.

## Out of scope for this milestone

The current code does not yet provide application virtualization, VM isolation, traffic routing, Tor integration, firewall configuration, MAC randomization, disk encryption, key storage, firmware replacement, verified boot, or system-log management. These must not be advertised as implemented.

## Adversaries and assumptions

Consider malicious websites and files, untrusted networks, compromised user-space applications, hostile or misconfigured local processes, supply-chain compromise, and accidental operator error. A fully compromised kernel, firmware, hypervisor, or physical device may defeat controls implemented only in user space.

The project cannot promise anonymity, prevent all IP leaks, or protect against every endpoint compromise. The real protection depends on OS version, privileges, configuration, hardware, and integration quality.

## Core invariants

1. A required protection that is inactive, degraded, or unknown must produce a deny decision.
2. Unknown or unsupported platform capabilities must never be presented as active.
3. UI status is not enforcement; the trusted platform adapter must enforce decisions.
4. Security failures must be visible and diagnosable; do not silently erase evidence.
5. No custom cryptographic algorithm or protocol should be introduced.
6. Every platform-specific feature needs negative-path tests, including crashes, disconnects, permission failures, and partial initialization.

## Next engineering gates

1. Define exact threat scenarios and platform support matrix.
2. Implement a narrow platform adapter behind explicit interfaces.
3. Test actual enforcement on each supported OS, including failure and recovery paths.
4. Review dependency provenance, permissions, update integrity, and release signing.
5. Commission independent security review before making strong protection claims.
