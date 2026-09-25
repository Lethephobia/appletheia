use super::CurrencyRegistrarInvitationDeclineRejectionReason;

/// Describes the domain outcome of an currency registrar invitation decline request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyRegistrarInvitationDeclineResult {
    Declined,
    Rejected {
        reason: CurrencyRegistrarInvitationDeclineRejectionReason,
    },
}
