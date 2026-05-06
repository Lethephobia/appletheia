use super::CurrencyIssuanceCompleteRejectionReason;

/// Describes the domain outcome of a currency issuance complete request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyIssuanceCompleteResult {
    Completed,
    Rejected {
        reason: CurrencyIssuanceCompleteRejectionReason,
    },
}
