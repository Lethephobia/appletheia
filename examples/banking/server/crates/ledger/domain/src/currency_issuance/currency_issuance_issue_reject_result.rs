use super::CurrencyIssuanceIssueRejectionReason;

/// Describes the domain outcome of a currency issuance rejection request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyIssuanceIssueRejectResult {
    Rejected {
        reason: CurrencyIssuanceIssueRejectionReason,
    },
}
