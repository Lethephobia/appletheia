use super::CurrencyRegistrarJoinRequestCancelRejectionReason;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyRegistrarJoinRequestCancelResult {
    Canceled,
    Rejected {
        reason: CurrencyRegistrarJoinRequestCancelRejectionReason,
    },
}
