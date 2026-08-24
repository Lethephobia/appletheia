use super::CurrencyRegistrarInvitationIssueRejectionReason;

/// Describes the domain outcome of an currency registrar invitation issue request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyRegistrarInvitationIssueResult {
    Issued,
    Rejected {
        reason: CurrencyRegistrarInvitationIssueRejectionReason,
    },
}
