use super::CurrencyRegistrarInvitationCancelRejectionReason;

/// Describes the domain outcome of an currency registrar invitation cancel request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyRegistrarInvitationCancelResult {
    Canceled,
    Rejected {
        reason: CurrencyRegistrarInvitationCancelRejectionReason,
    },
}
