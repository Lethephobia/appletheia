use super::CurrencyRegistrarInvitationAcceptRejectionReason;

/// Describes the domain outcome of an currency registrar invitation accept request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyRegistrarInvitationAcceptResult {
    Accepted,
    Rejected {
        reason: CurrencyRegistrarInvitationAcceptRejectionReason,
    },
}
