use super::CurrencyRegistrarJoinRequestRejectRejectionReason;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyRegistrarJoinRequestRejectResult {
    Rejected,
    RejectionRejected {
        reason: CurrencyRegistrarJoinRequestRejectRejectionReason,
    },
}
