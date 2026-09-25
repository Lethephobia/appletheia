use super::CurrencyRegistrarCreateRejectionReason;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyRegistrarCreateResult {
    Created,
    Rejected {
        reason: CurrencyRegistrarCreateRejectionReason,
    },
}
