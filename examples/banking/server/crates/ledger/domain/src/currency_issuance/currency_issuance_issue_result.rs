use super::CurrencyIssuanceIssueRejectionReason;

/// Describes the domain outcome of a currency issuance request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyIssuanceIssueResult {
    Issued,
    Rejected {
        reason: CurrencyIssuanceIssueRejectionReason,
    },
}
