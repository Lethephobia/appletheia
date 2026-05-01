use super::CurrencyIssuanceFailRejectionReason;

/// Describes the domain outcome of a currency issuance fail request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyIssuanceFailResult {
    Failed,
    Rejected {
        reason: CurrencyIssuanceFailRejectionReason,
    },
}
