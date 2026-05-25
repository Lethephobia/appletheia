use super::CurrencySupplyCommitRejectionReason;

/// Describes the domain outcome of a commit-supply request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencySupplyCommitResult {
    Committed,
    Rejected {
        reason: CurrencySupplyCommitRejectionReason,
    },
}
