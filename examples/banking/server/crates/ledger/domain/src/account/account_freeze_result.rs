use super::AccountFreezeRejectionReason;

/// Describes the domain outcome of a freeze request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccountFreezeResult {
    Frozen,
    Rejected {
        reason: AccountFreezeRejectionReason,
    },
}
