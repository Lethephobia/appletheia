use super::OwnedAccountClosureFailRejectionReason;

/// Returned after failing an owned account closure.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OwnedAccountClosureFailResult {
    Failed,
    Rejected {
        reason: OwnedAccountClosureFailRejectionReason,
    },
}
