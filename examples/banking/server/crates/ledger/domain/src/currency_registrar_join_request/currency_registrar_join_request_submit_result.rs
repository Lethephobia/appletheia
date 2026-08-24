use super::CurrencyRegistrarJoinRequestSubmitRejectionReason;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyRegistrarJoinRequestSubmitResult {
    Submitted,
    Rejected {
        reason: CurrencyRegistrarJoinRequestSubmitRejectionReason,
    },
}
