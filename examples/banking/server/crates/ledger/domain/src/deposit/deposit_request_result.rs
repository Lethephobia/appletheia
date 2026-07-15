use super::DepositRequestRejectionReason;

/// Describes the domain outcome of a deposit request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DepositRequestResult {
    Requested,
    Rejected {
        reason: DepositRequestRejectionReason,
    },
}
