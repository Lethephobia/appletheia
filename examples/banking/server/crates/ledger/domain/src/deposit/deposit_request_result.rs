use super::{DepositId, DepositRequestRejectionReason};

/// Describes the domain outcome of a deposit request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DepositRequestResult {
    Requested {
        deposit_id: DepositId,
    },
    Rejected {
        deposit_id: DepositId,
        reason: DepositRequestRejectionReason,
    },
}
