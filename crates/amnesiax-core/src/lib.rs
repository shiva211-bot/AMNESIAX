//! Minimal policy primitives for AMNESIAX.
//!
//! This crate models decisions only. It does not enforce network controls at
//! the operating-system or hardware layer. Callers must not treat a decision
//! returned here as proof that traffic is actually blocked or routed safely.

/// The observed state of a required network protection mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkGuardState {
    /// The guard is active and its adapter has confirmed that state.
    Active,
    /// The guard is disabled.
    Disabled,
    /// The guard is present but unhealthy or unable to guarantee protection.
    Degraded,
    /// The state could not be reliably determined.
    Unknown,
}

/// The result of a policy decision about a network operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkDecision {
    Allow,
    Deny,
}

/// A capability's implementation status on the current platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityStatus {
    Supported,
    Unsupported,
    Unknown,
}

/// Decide whether a network operation may proceed.
///
/// If require_guard is true, access is allowed only when the guard is
/// explicitly reported as active. Every other state denies access.
///
/// This is a policy decision, not an OS firewall rule. The caller must enforce
/// the result at a trusted boundary and test that enforcement independently.
pub fn decide_network_access(
    require_guard: bool,
    guard_state: NetworkGuardState,
) -> NetworkDecision {
    if !require_guard || guard_state == NetworkGuardState::Active {
        NetworkDecision::Allow
    } else {
        NetworkDecision::Deny
    }
}

/// Return true only when a capability is both supported and confirmed enabled.
///
/// Unknown support or a caller-provided enabled flag is never enough on its
/// own to advertise a capability as active.
pub fn is_capability_confirmed_enabled(
    status: CapabilityStatus,
    enabled: bool,
) -> bool {
    status == CapabilityStatus::Supported && enabled
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_guard_allows_only_confirmed_active_state() {
        assert_eq!(
            decide_network_access(true, NetworkGuardState::Active),
            NetworkDecision::Allow
        );
        for state in [
            NetworkGuardState::Disabled,
            NetworkGuardState::Degraded,
            NetworkGuardState::Unknown,
        ] {
            assert_eq!(
                decide_network_access(true, state),
                NetworkDecision::Deny,
                "required guard state {:?} must fail closed",
                state
            );
        }
    }

    #[test]
    fn optional_guard_does_not_block_access() {
        for state in [
            NetworkGuardState::Active,
            NetworkGuardState::Disabled,
            NetworkGuardState::Degraded,
            NetworkGuardState::Unknown,
        ] {
            assert_eq!(
                decide_network_access(false, state),
                NetworkDecision::Allow
            );
        }
    }

    #[test]
    fn capability_requires_support_and_confirmation() {
        assert!(is_capability_confirmed_enabled(CapabilityStatus::Supported, true));
        assert!(!is_capability_confirmed_enabled(CapabilityStatus::Supported, false));
        assert!(!is_capability_confirmed_enabled(CapabilityStatus::Unsupported, true));
        assert!(!is_capability_confirmed_enabled(CapabilityStatus::Unknown, true));
        assert!(!is_capability_confirmed_enabled(CapabilityStatus::Unknown, false));
    }
}
