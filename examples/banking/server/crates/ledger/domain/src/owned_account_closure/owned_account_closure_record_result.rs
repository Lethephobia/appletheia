use super::OwnedAccountClosureRecordRejectionReason;

/// Returned after recording owned account closure progress.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OwnedAccountClosureRecordResult {
    Recorded,
    Rejected {
        reason: OwnedAccountClosureRecordRejectionReason,
    },
}
