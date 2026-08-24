use super::CurrencyRegistrarJoinRequestApproveRejectionReason;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyRegistrarJoinRequestApproveResult {
    Approved,
    Rejected {
        reason: CurrencyRegistrarJoinRequestApproveRejectionReason,
    },
}
