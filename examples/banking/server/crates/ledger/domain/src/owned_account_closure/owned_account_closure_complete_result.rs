use super::OwnedAccountClosureCompleteRejectionReason;

/// Returned after completing an owned account closure.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OwnedAccountClosureCompleteResult {
    Completed,
    Rejected {
        reason: OwnedAccountClosureCompleteRejectionReason,
    },
}
