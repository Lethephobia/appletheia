use super::PayoutDestinationRemoveRejectionReason;

/// Describes the domain outcome of removing a payout destination.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PayoutDestinationRemoveResult {
    Removed,
    Rejected {
        reason: PayoutDestinationRemoveRejectionReason,
    },
}
