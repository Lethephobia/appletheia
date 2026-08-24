use super::CurrencyRegistrarHandleChangeRejectionReason;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyRegistrarHandleChangeResult {
    Changed,
    Rejected {
        reason: CurrencyRegistrarHandleChangeRejectionReason,
    },
}
