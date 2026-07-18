use super::CurrencyDefineRejectionReason;

/// Describes the domain outcome of a currency define request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyDefineResult {
    Defined,
    Rejected {
        reason: CurrencyDefineRejectionReason,
    },
}
