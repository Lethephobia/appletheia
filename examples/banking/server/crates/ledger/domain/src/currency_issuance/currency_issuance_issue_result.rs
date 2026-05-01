use super::{CurrencyIssuanceId, CurrencyIssuanceIssueRejectionReason};

/// Describes the domain outcome of a currency issuance request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyIssuanceIssueResult {
    Issued {
        currency_issuance_id: CurrencyIssuanceId,
    },
    Rejected {
        reason: CurrencyIssuanceIssueRejectionReason,
    },
}
